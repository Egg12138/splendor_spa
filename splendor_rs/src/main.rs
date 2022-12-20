
#![cfg_attr(test, feature(test))]
mod utils;
mod app;
mod cards;
mod gems;
mod player;
mod nobles;
pub mod test;
use utils::{fileio, shuffle};



fn main() {



	unsafe {
		let cards_pool  = fileio::read_into_cardspool("../cards.csv").unwrap_unchecked();
		println!("{:?}", cards_pool);
		let nobles = fileio::read_into_noblespool("../nobles.csv").unwrap_unchecked();
		println!("{:?}", nobles);
		//let nobles_pool: Vec<nobles::Noble> = uitls::fileio::read_into_noblespool("../nobles.csv").unwrap_unchecked();
	}
}

