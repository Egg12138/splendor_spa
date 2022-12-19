#![allow(dead_code)]


use crate::utils::{shuffle, self};
use crate::app::{
	Color, DECK1, DECK2, DECK3,
};
use serde::{Deserialize, Serialize};
use std::io::Result;
use std::collections::HashMap;



/// **WARN**: 我们尽量保证Card实例是不可变的！ 
/// 因为在游戏中 Card 信息是不变的，且处于安全性（开发时也节省精力），我们将限制 Card 的可变性
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Clone)]
pub struct Card {
	score: u8,
	color: Color,
	// cost:  [u8; 5], 
	cost: GemNumMap,
	level: u8,
	// id: Cid,
}

impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    writeln!(f, "lv{}{}[{}] {}", self.level, self.color, self.score, self.cost)
	}
}



impl Card {


/// provide a demo card instant: score = 2, color = user defined, cost = default costmap.
	pub fn demo(clr: Color) -> Self {
			Card {
				score: 2,
				color: clr, 
				cost: GemNumMap::default(), 
				level: 2,
			}
		} 

	pub fn new(score: u8, color: Color, cost: GemNumMap, level: u8) -> Self {
		Card {
			score,
			color,
			cost,
			level
		}	
	}

	/// 由于有数据转换，我们在这里返回`Result<Card>`而非 raw `Card`
	pub fn from_arr(arg_tuple: [u8; 8]) -> Result<Card> {
		let score = arg_tuple[0];
		let color = arg_tuple[1];
		let color = utils::handler::num_to_color(color);
		// let costmap = CostMap::from_arr_ref(&arg_tuple[2..6]);
		let costs: &[u8; 5] = arg_tuple[2..7].try_into()
			.expect("Converting arg_tuple[2..7] into &[u8;5] failed." );
		let cost = GemNumMap::from_arr_ref(&costs);
		let level = arg_tuple[7]; 
		Ok(Card {
			score,
			color,
			cost,
			level,
		})
	}


	pub fn level(&self) -> u8 {
		self.level
	} 

	pub fn score(&self) -> u8 {
		self.score
	}

	pub fn color(&self) -> Color {
		self.color.clone()
	}

	pub fn cost(&self) -> GemNumMap {
		self.cost.clone()
	}

}




/**
 * @brief      { 存储一张卡的花费的结构(Unstable) }
 */
/// `pub struct CostMap` 直接封装了一个 HashMap.当然处于简单之后CostMap也可以直接被替换为一个一维向量来作为卡牌开销的存储
/// 作为一个封装 HashMap的结构体，它有若干便捷之处（性能上不确定，不知道强化学习这个数据结构的性能敏感度如何）
/// * 安全地访问 HashMap的键值对
/// * 提供了pretty-print
/// * 提供了（或许有用的）返回非零价格（也就是实际的卡牌的价格栏出现的信息）
/// * 提供了转化为一个Vec的接口
/// * 可以使用Eq比较
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GemNumMap {
	pub map: HashMap<Color, u8>, 
}

impl Default for GemNumMap {
	fn default() -> Self {
		let costs =	GemNumMap {
			map: HashMap::from([
			 (Color::Black, 0u8), 
			 (Color::Blue, 0u8), 
			 (Color::Green, 0u8), 
			 (Color::Red, 0u8), 
			 (Color::White, 0u8),
			])
		};
		costs
}
}



impl std::fmt::Display	for GemNumMap	 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let mut strs = String::new();
			let colors = Vec::from([Color::Black, Color::Blue, Color::Green, Color::Red, Color::White]);
			for clr in colors	{
				if self.contains_key(&clr) {
					strs.push_str(&format!("{}{} ", self.get_cost(clr).unwrap(), clr));
				}
			}
			write!(f, "{}", strs)
		}
}


impl GemNumMap {	
	pub fn new() -> Self {
		GemNumMap { map: HashMap::new() }
	}

	// IMPL: Finish the from arr function
	/// `from_arr_ref`没必要做`Result`检查，因为对于只有五个字段的`CostMap`，源码选择直接对`arg_arr[0]`->`arg_arr[4]`枚举实现
	pub fn from_arr_ref(arg_arr: &[u8; 5]) -> GemNumMap {
		let mut costmap =  GemNumMap::new();
		costmap.insert(Color::Black, arg_arr[0])
			.insert(Color::Blue, arg_arr[1])
			.insert(Color::Green, arg_arr[2])
			.insert(Color::Red, arg_arr[3])
			.insert(Color::White, arg_arr[4]);
		costmap.clone()
	}

	pub fn get_cost(&self, color: Color) -> Option<&u8> {
		self.map.get(&color)
	}

	pub fn total_gemsnum(&self) -> u8 {
		let result: u8 = self.map.values().map(|&v| v).sum();
		result	
	}

	pub fn colors(&self) -> std::collections::hash_map::Keys<Color, u8>	 {
		self.map.keys()
	}

	/// 
	/// 直接返回一个&u8数组，作为最简单的一种获得指定宝石开销的方式
	/// 由于HashMap的key是无需存储的，所以我们若使用这种方式获得`Vec<&u8>`仅仅是计数用
	#[deprecated]
	pub fn get_cost_vec(&self) -> Vec<&u8> {
		let costs: Vec<&u8> = self.map.values().collect();
		costs
	} 

	/// 返回自身的Option封装
	pub fn get_cost_map(&self) -> Option<&GemNumMap> {
		Some(self)
	}


	///
	/// append a new K-V pair or update an old value of a present key.
	/// # Examples
	///
	/// ```rust
	/// use splendor_rs::{ Color, CostMap};
	/// 
	/// let mut cost_map = CostMap::new();
	/// let color = Color::Red;
	/// let color1 = Color::Green;
	/// let cost = 2;
	/// let cost1 = 3;
	/// //你可以这样：
	/// let result = cost_map
	///        .insert(color, cost)
	///        .insert(color1, cost1);
	/// assert_eq!(result.len(), 2);
	/// ```
	pub fn insert(&mut self, color: Color, cost: u8) -> &mut GemNumMap {
			self.map.insert(color, cost);
			self
		}

	pub fn len(&self) -> usize {
		self.map.len()
	}

	/// 获得非零价格的宝石开销。
	/// 这个函数的实现比较笨拙，对性能有不少的额外开销，**慎重使用**
	pub fn nonzero_costs(&mut self) -> GemNumMap {
			let mut res = GemNumMap::new();
			for (key, &mut val) in self.map.iter_mut() {

				if val > 0 {
					let clr = key.clone();
					println!("{}=>{}", clr, val);
					res.insert(clr, val);
				} 
				
			}
			res
		}

	pub fn contains_key(&self, key: &Color) -> bool {
		self.map.contains_key(key)
	}


	/// 把某色数量递增1, 返回原值
	pub fn increment1(&mut self, clr: Color) {
		let origin = self.get_cost(clr).unwrap();
		self.insert(clr, *origin + 1);
	}

	/// 即将移除, 目前看好像用不到这个？
	#[deprecated]
	pub fn remove(&mut self, key: &Color) -> Option<u8> {
		self.map.remove(key)
	} 

	/// # Output:
	/// like this:
	/// ```text
	/// > 0💎 7💎 9🔴 0🟢 0⚪ 0🟤
	/// ```
	pub fn pretty_print(&mut self) {
		for (k, v) in self.map.iter_mut() {
			print!("{v}{k} ");
		}
		print!("\n");
	}
}



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Deck {
	level: u8,
	pub rest_decks: Vec<Card>, 
	capacity: usize,
	len: usize,
}


impl Deck {

	pub fn new(level: u8) -> Self {
		Deck {
			level,
			rest_decks: Vec::new(),
			capacity: match level { 1 => DECK1, 2 => DECK2, 3 => DECK3, _ => 0},
			len: 1,
		}

	}

	pub fn level(&self) -> u8 {
		self.level
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn rest_len(&self) -> usize {
		self.len
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	#[inline]
	pub fn pop(&mut self) -> Option<Card> {
		if self.is_empty() {
			None
		} else {
	// TODO: Finish the return card
			self.len -= 1;
			let _index = self.rest_decks.pop().unwrap();
			Some(Card::demo(Color::Black))
			// Some(cards_pool.get(card_index))
		}
	}

	#[inline]
	pub fn pop4(&mut self) -> Option<(Card, Card, Card, Card)> {
		if self.len >= 4 {
			Some((self.pop().unwrap(), self.pop().unwrap(), self.pop().unwrap(), self.pop().unwrap()))	
		} else {
			None
		}
	}

	pub fn push(&mut self, card: Card) {
		self.rest_decks.push(card);
	}

	pub fn shuffle(&mut self) {
		shuffle::shuffle_deck(self);
	}

}

