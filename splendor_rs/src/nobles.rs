use serde::{Deserialize, Serialize};
use crate::cards::CostMap;
use crate::app::{Nid, Cid, NUMPLAYERS, NUMNOBLES};



#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Noble {
	pub requirement: CostMap,
	pub id: Nid,
}

impl Noble {
	// TODO: 搞一个正确的贵族demo信息
	pub fn demo() -> Self {
		Noble { 
			requirement: CostMap::from_arr_ref(&[0u8, 0u8, 2u8, 1u8, 1u8]), 
			id: 3 
		}
	}
}


/// 表示游戏过程中洗出来的贵族数，默认实现双人游戏，也就是长度为3
#[derive(PartialEq, Eq)]
pub struct NobleList {
	nobles: Vec<Noble>,
	len: usize,
	capacity: usize,
}


impl NobleList {

	pub fn empty() -> Self {
		NobleList { nobles: Vec::new(), len: 0, capacity: NUMNOBLES }
	}

	pub fn new(noble_arr: [Noble; NUMNOBLES]) -> Self {
		NobleList { nobles: Vec::from(&noble_arr[..]), len: NUMNOBLES, capacity: NUMNOBLES }
	}

	fn is_empty(&self) -> bool {
		self.len == 0
	}
	pub fn push(mut self, noble: Noble) -> NobleList {
		if self.len >= 1 {
			self.nobles.push(noble);
			self.len -= 1;
		}
		self
	}

	pub fn get_all_nids(&self) -> Option<Vec<Nid>> {
		if self.is_empty() {
			None
		} else {
			let nids = self.nobles.iter().map(|n| n.id).collect::<Vec<Nid>>();
			Some(nids)
		} 
	} 

	pub fn get_requirement(&self, idx: usize) -> Option<CostMap> {
		if idx >= self.len {
			None
		} else {
			Some(self.nobles[idx].requirement.clone())
		}
	} 

	// pub fn remove(&mut self, )


}
