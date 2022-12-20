// tool functions

/// 提供一些公用的处理数据的函数
use crate::app::*;
use crate::cards::{ GemNumMap, Card};
use crate::nobles::Noble;
use crate::player::Player;
use super::compiled_data::{self, CARDSARR_POOL, NOBLESARR_POOL};
pub(crate) fn is_over(player_state: Player) -> bool {

		false

}

pub(crate) fn num_to_color(num: u8) -> Color {
	let color =  match num {
			1 => Color::Black,
			2 => Color::Blue,
			3 => Color::Green,
			4 => Color::Red,
			5 => Color::White,
			_ => Color::Gold,
		};
	color
} 


#[inline]
pub(crate) fn get_cardspool() -> Vec<Card> {
	let deck: Vec<Card> = CARDSARR_POOL.into_iter().map(|arr| Card::from_arr(arr).unwrap_or_default()).collect();
	deck
}
#[inline]
pub(crate) fn get_noblespool() -> Vec<Noble> {
	let deck: Vec<Noble> = NOBLESARR_POOL.into_iter().map(|arr| Noble::from_arr(arr)).collect();
	deck
}