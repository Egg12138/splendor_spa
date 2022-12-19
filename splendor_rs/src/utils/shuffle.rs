// shuffle.rs

use crate::cards::{Card, Deck};
use crate::app::{Id, Color};
use std::path::Path;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle_deck(deck: &mut Deck) {
	// NOTICE Checkout the thread security
	let mut rng = thread_rng();
	deck.rest_decks.shuffle(&mut rng);
}

