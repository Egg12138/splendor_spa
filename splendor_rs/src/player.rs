use crate::nobles::Noble;
use crate::cards::{Card, GemNumMap};
use crate::app::{ Id, Color};
use std::error::Error;
use serde::{Deserialize, Serialize};

pub enum Action {
	Monochromatic,
	Single,
	Reserve,
	Buy,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Player {

	pub id: Id, 		//0，1，轮次 
	pub reserved: Vec<Card>,
	pub bought: Vec<Card>, 
	pub gems: GemNumMap,
	pub score: usize,	
	pub gold_num: usize,
}


impl Player {
	/// 二人对战。id为0或1.
	pub fn init(id: Id) -> Self {
		Player { id, reserved: Vec::new(), bought: Vec::new(), gems: GemNumMap::default(), score: 0 , gold_num: 0}
	}	

	pub fn can_reserve(&self) -> bool {	self.reserved.len() < 3	}
	pub fn reach_target(&self) -> bool { self.score >= 5}
	pub fn round_head(&self) -> bool { self.id == 0}

	pub fn reserve_one(&mut self, reserved_card: Card, gold_num: usize) {
		self.reserved.push(reserved_card);
		self.gold_num += gold_num;
	}

	pub fn bonus(&mut self, score: usize) {
		self.score += score;
	}

	/// 在判定操作合法后使用，判定逻辑需要参考`struct crate::app::GameSM`
	pub fn bought_one(&mut self, bought: Card) {
		let bonus = bought.score().into();
		self.bought.push(bought);
		self.bonus(bonus);
	}

	pub fn get_a_noble(&mut self) {
		self.bonus(3);
	}

	/// 返回下一次最多可拿取的宝石数量, 最大为3
	pub fn once_pick_max(&self) -> u8 {
		let used = self.gems_num();
		if used <= 7 { 3 } else { 10 - used  }
	}

	fn gems_num(&self) -> u8 {
		self.gems.total_gemsnum()
	}	

	/// 判定合法操作后的，往库存池中添加一个宝石的颜色，返回这个颜色原来的数量的`Option`封装
	/// 不直接返回`u8`.是考虑到`CostMap::get_cost`可能出现错误
	pub fn picked_gem(&mut self, gemcolor: Color){
			self.gems.increment1(gemcolor)
	}

	/// 传入 costmap 数组`[u8; 5]`, 返回总共减少的宝石开销
	/// 解析错误返回"Spending gems err."
	pub fn spend_by_array(&mut self, arr: [u8; 5]) -> usize {
			let costmap = GemNumMap::from_arr_ref(&arr);
			self.spend_by_costmap(costmap)

	}
	/// 不对外暴露该接口
	fn spend_by_costmap(&mut self, costmap: GemNumMap) -> usize {
		let sum = costmap.total_gemsnum();
		for &clr in costmap.colors() {
			let new_val = self.gems.get_cost(clr).unwrap() - costmap.get_cost(clr).unwrap();
			self.gems.insert(clr, new_val);
		}
		sum.into()
	}

	




}





