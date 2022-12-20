// lib.rs 
// define data structs
// TODO: 一定找到一个存储HashMap的好的方式
// 我们总是存储卡牌和贵族的Cid/Nid
// 一些基本的数据结构
#![allow(dead_code)]

use crate::nobles::Noble;
use crate::cards::Deck;
use crate::player::Player;
use crate::cards::GemNumMap;
use serde::{Deserialize, Serialize};
use core::fmt;
// use std::convert::TryInto;
use std::fmt::Display;

// id of a card
pub type Id = usize;


// NUMNOBLE, NUMPLAYER之后都可以改成命令行参数，但目前没必要
pub const NUMNOBLES: usize = 3;
pub const NUMPLAYERS: usize = 2; 
pub const NUMCARDS: usize =90;

pub const DECK1NUM: usize = 40;
pub const DECK2NUM: usize = 30;
pub const DECK3NUM: usize = 20;

pub const LEVEL_IDX: usize = 7;

pub const BLUE: &str = "\u{1F48E}";
pub const RED: &str = "\u{1F534}";
pub const BLACK: &str = "\u{1F7E4}";
pub const GREEN: &str = "\u{1F7E2}";
pub const WHITE: &str = "\u{26AA}";
pub const GOLD: &str = "\u{1F48E}";

// Make a solution for the cards pool

#[derive(PartialEq, Debug, Serialize, Deserialize, Hash, Eq, Clone, Copy)]
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

// TODO: 可以使用compiled_data来创建全局数据了

/// 游戏状态机
/// 我们不希望不断地将此状态机传递，所以主要是在状态机内引入玩家结构体和可变区域
/// 总的牌池数据不会存在在游戏状态机里
#[derive(Debug, Clone)]
pub struct GameSM {
	pub round: usize,
	/// deck of level Nth developement cards.长度分别为40,30,20
	pub deck1: Deck,
	pub deck2: Deck,
	pub deck3: Deck,
	pub uncovered1: Deck,
	pub uncovered2: Deck,
	pub uncovered3: Deck,
	pub playerlists: Vec<Player>,
	pub gemsrested: GemNumMap, 
	pub goldrested: usize,
	pub avaliable_nobles: Vec<Noble>,
}

impl GameSM {
	/// `GameSM::init`创建了一个部分未初始的游戏状态机 
	pub fn init() -> Self {
		GameSM {
			round: 0,
			deck1: Deck::new(1),
			deck2: Deck::new(2),
			deck3: Deck::new(3),
			uncovered1: Deck::new(1),
			uncovered2: Deck::new(2),
			uncovered3: Deck::new(3),
			playerlists: Vec::from([Player::init(0), Player::init(1)]),
			gemsrested: GemNumMap::from_arr_ref(&[7,7,7,7,7]),
			goldrested: 5,
			avaliable_nobles: Vec::new(),
		}	
	}




}


