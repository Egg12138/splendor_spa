// shuffle.rs

use crate::cards::Deck;



use rand::seq::SliceRandom;
use rand::thread_rng;

/// 安全的，且不需要返回`Result`地进行洗牌操作
/// 在这里使用安全的内存转换，（启用越界检查的下表对换）
pub fn shuffle_deck(deck: &mut Deck) {
	// NOTICE Checkout the thread security
	let mut rng = thread_rng();
	deck.rest_decks.shuffle(&mut rng);
}

