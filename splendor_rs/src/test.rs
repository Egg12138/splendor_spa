// test for lib
// TODO: Need to be re-constructed
// 
use crate::app::*;
use crate::utils;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn colors_pretty_print() {
		struct RedGem {
			color: Color,
			price: u32,
		}	

		let gem = RedGem { color: Color::Red, price: 114514 };
		println!("{}", gem.color);
		let black = Color::Black;
		let blue = Color::Blue;
		let red = Color::Red;
		let white = Color::White;
		let gold = Color::Gold;
		let green = Color::Green;
		println!("{}{}{}{}{}{}", blue, black, red, white, green, gold);
	}

	#[test]
	fn cost_map_works() {
		let mut cost = CostMap::default(); 
		let black = Color::Black;
		let blue = Color::Blue;
		let red = Color::Red;
		let white = Color::White;
		let gold = Color::Gold;
		let green = Color::Green;
		assert!(cost.contains_key(&black));
		assert!(cost.contains_key(&red));
		assert!(cost.contains_key(&blue));
		assert!(cost.contains_key(&white));
		cost.insert(gold, 1);
		let gold = Color::Gold;
		assert!(cost.contains_key(&gold));
		assert!(cost.contains_key(&green));
		assert!(cost.contains_key(&red));
		assert_eq!(cost.len(), 6);

		assert_eq!(cost.get_cost(black), Some(&0));
		cost.remove(&Color::Gold);
		cost.remove(&Color::Red);
		assert_eq!(cost.get_cost(gold), None);

		cost.insert(Color::Gold, 7)
			.insert(Color::Red, 9);
		let positive = cost.nonzero_costs();
		println!("{:#?}", positive);
		cost.pretty_print();
		assert_eq!(positive.len(), 2);
		assert!(positive.contains_key(&Color::Gold));
		assert!(positive.contains_key(&Color::Red));


	} 

	fn cost_map_get_works() {
		let costs = CostMap::default();
		if let Some(get_costs) = costs.get_cost_map() {
			assert_eq!(get_costs.len(), 5);
		}
		let get_costs_vec = costs.get_cost_vec();
		assert_eq!(get_costs_vec.len(), 5);
	}

	fn cost_vec_works() {
		todo!()
	}

	fn card_construct_works() {
		unimplemented!()
	}

	#[test]
	fn shuffle_works() {
		use rand::seq::SliceRandom;

		let mut deck = Deck::new(1);
		let cid: u8 = rand::random();
		deck.push(cid);	
		let cid: u8 = rand::random();
		deck.push(cid);	
		let cid: u8 = rand::random();
		deck.push(cid);	
		deck.shuffle();
		println!("{:?}", deck.rest_decks);
	}

	#[test]
	fn deck_shuffle_works() {




	}



}