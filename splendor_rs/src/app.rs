// lib.rs 
// define data structs
// TODO: 一定找到一个存储HashMap的好的方式
// 我们总是存储卡牌和贵族的Cid/Nid
// 一些基本的数据结构
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use core::fmt;
// use std::convert::TryInto;
use std::fmt::Display;

// id of a card
pub type Cid = usize;
// id of a noble
pub type Nid = usize;


// NUMNOBLE, NUMPLAYER之后都可以改成命令行参数，但目前没必要
pub const NUMNOBLES: usize = 3;
pub const NUMPLAYERS: usize = 2; 
pub const NUMCARDS: usize =90;

pub const BLUE: &str = "\u{1F48E}";
pub const RED: &str = "\u{1F534}";
pub const BLACK: &str = "\u{1F7E4}";
pub const GREEN: &str = "\u{1F7E2}";
pub const WHITE: &str = "\u{26AA}";
pub const GOLD: &str = "\u{1F48E}";

// Make a solution for the cards pool

#[derive(PartialEq, Debug, Serialize, Deserialize, Hash, Eq, Clone, Copy	)]
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


