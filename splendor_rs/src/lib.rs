// lib.rs 
// define data structs
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use core::fmt;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;


const BLUE: &str = "\u{1F48E}";
const RED: &str = "\u{1F534}";
const BLACK: &str = "\u{1F7E4}";
const GREEN: &str = "\u{1F7E2}";
const WHITE: &str = "\u{26AA}";
const GOLD: &str = "\u{1F48E}";


#[derive(PartialEq, Debug, Serialize, Deserialize, Hash, Eq, Clone)]
pub enum Color {
	Black,
	Blue,
	Green,
	Red,
	White,
	Gold
} 

impl Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Black => { write!(f,"{}", BLACK)},
			Self::Blue => { write!(f, "{}", BLUE)},
			Self::Green => { write!(f, "{}", GREEN)},
			Self::Red => { write!(f, "{}", RED)},
			Self::White => {write!(f, "{}", WHITE)},
			Self::Gold => { write!(f, "{}", GOLD)},
		}
	}
}


/// **WARN**: 我们尽量保证Card实例是不可变的！ 
/// 因为在游戏中 Card 信息是不变的，且处于安全性（开发时也节省精力），我们将限制 Card 的可变性
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
	score: u8,
	color: Color,
	// cost:  [u8; 5], 
	cost: CostMap,
	level: u8,
}



impl Card {


/// provide a demo card instant: score = 2, color = user defined, cost = default costmap.
	pub fn demo(clr: Color) -> Self {
			Card {
				score: 2,
				color: clr, 
				cost: CostMap::default(), 
				level: 2,
			}
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

	pub fn cost(&self) -> CostMap {
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CostMap {
	map: HashMap<Color, u8>, 
}
impl Default for CostMap {
	fn default() -> Self {
		let costs =	CostMap {
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

impl CostMap {	
	pub fn new() -> Self {
		CostMap { map: HashMap::new() }
	}

	pub fn get_cost(&self, color: Color) -> Option<&u8> {
		self.map.get(&color)
	}

	/// 
	/// 直接返回一个&u8数组，作为最简单的一种获得各种宝石开销的方式
	/// Vec[Idx]: 我们可以按照Enum Color的顺序来检索:Black => idx:0, Bluue => idx: 1, ...
	pub fn get_cost_vec(&self) -> Vec<&u8> {
		let costs: Vec<&u8> = self.map.values().collect();
		costs
	} 

	/// 返回自身的Option封装
	pub fn get_cost_map(&self) -> Option<&CostMap> {
		Some(self)
	}

	///
	/// # Examples
	///
	/// ```rust
	/// use splendor_rs::CostMap;
	///
	/// let mut cost_map = Costap::new();
	/// /* ---snip---  */
	/// //你可以这样：
	/// let result = cost_map
	/// 		.insert(color, cost)
	/// 		.insert(color1, cost1);
	/// assert_eq!(result.len(), 2);
	/// ```
	pub fn insert(&mut self, color: Color, cost: u8) -> &mut CostMap {
			self.map.insert(color, cost);
			self
		}

	pub fn len(&self) -> usize {
		self.map.len()
	}

	/// 获得非零价格的宝石开销。
	/// 这个函数的实现比较笨拙，对性能有不少的额外开销，**慎重使用**
	pub fn nonzero_costs(&mut self) -> CostMap{
			let mut res = CostMap::new();
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
		println!("");
	}
}