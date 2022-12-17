// shuffle.rs

use crate::app::{Color, Card, Deck};
use std::path::Path;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;


type Cid = u16;
pub fn get_cards_from_csv<P: AsRef<Path>>(path: P) -> Vec<Cid> {
	let content = fs::File::open(path);
	let card_ids = vec![1];
	card_ids
} 


pub fn shuffle_deck(deck: &mut Deck) {
	// NOTICE Checkout the thread security
	let mut rng = thread_rng();
	deck.rest_decks.shuffle(&mut rng);
}