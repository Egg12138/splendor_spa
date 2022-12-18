mod utils;
mod app;
mod cards;
mod gems;
mod player;
mod nobles;
pub mod test;
use utils::shuffle;

use crate::cards::CardPool;


fn main() {

	// if let Ok(cards_pool) = CardPool::from(cards_pool_raw) {
		
	// }

	let cards_pool: Vec<cards::Card> = utils::fileio::read_into_cardspool("../cards.csv").unwrap();
	println!("{:?}", cards_pool);
}

