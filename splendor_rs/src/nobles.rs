#![allow(deprecated)]
use serde::{Deserialize, Serialize};
use crate::cards::GemNumMap;
use crate::app::NUMNOBLES;



#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Noble {
	pub requirement: GemNumMap,
	// pub id: Nid,
}

impl Default for Noble	{
	fn default() -> Self {
	    	Noble { 
	    		requirement: GemNumMap::default() 
	    	}
	}
}

impl std::fmt::Display for Noble {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    	write!(f, "Noble[ {}]", self.requirement)
	}

}

impl Noble {
	// TODO: 搞一个正确的贵族demo信息
	pub fn demo() -> Self {
		Noble { 
			requirement: GemNumMap::from_arr_ref(&[0u8, 0u8, 2u8, 1u8, 1u8]), 
			// id: 3 
		}
	}


	pub fn get_requirement(&self) -> GemNumMap {
		self.requirement.clone()
	}
	// TODO: 同样实现一个from_arr
	/// 和`cards::Card`一样，都是`from_arr`而不是`from_arr_ref`。
	/// 我们是打算直接将其设计为拿走所有权的形式
	pub fn from_arr(arr: [u8; 5]) -> Noble {
		Noble { requirement: GemNumMap::from_arr_ref(&arr) }
	}

	pub fn from_arr_requiremap(require_map: GemNumMap) -> Noble {
		Noble { requirement: require_map }
	}

}


/// 表示游戏过程中洗出来的贵族数，默认实现双人游戏，也就是长度为3
/// 限制了Vec的一些操作。
/// 之后还是换回Vec<Noble>吧
#[deprecated]
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


	pub fn get_requirement(&self, idx: usize) -> Option<GemNumMap> {
		if idx >= self.len {
			None
		} else {
			Some(self.nobles.get(idx).unwrap().get_requirement())
		}
	} 

	// pub fn remove(&mut self, )


}
