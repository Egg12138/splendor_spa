#![allow(dead_code)]
use crate::app::*;
use crate::utils;
use crate::cards::*;
use crate::nobles::*;
use crate::gems::*;

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
	fn costmap_partialeq() {

		let mut costs = CostMap::default();
		let new_color = Color::Blue;
		let new_black = Color::Black;
		costs.insert(new_color, 3);
		costs.insert(new_black, 2);
		assert_eq!(&mut costs, CostMap::default()
							.insert(Color::Blue, 3)
							.insert(Color::Black, 2)
				);
		assert_eq!(&mut costs.nonzero_costs(), CostMap::new()
							.insert(Color::Blue, 3)
							.insert(Color::Black, 2)
				);
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


	#[ignore]
	fn cost_vec_works() {
		todo!()
	}

	#[ignore]
	fn card_construct_works() {
		unimplemented!()
	}

	#[test]
	fn card_and_noble_partialeq_works() {
		let card = Card::from_tuple([1, 1, 0, 0, 2, 1, 1, 1]).unwrap();
		let noble = Noble::demo();
		let cmp_costmap = CostMap::from_arr_ref(&[0u8, 0u8, 2u8, 1u8, 1u8]);
		assert_eq!(noble, Noble { requirement: cmp_costmap.clone(), id: 3 });	
		assert_eq!(card, Card::new(1, Color::Blue, cmp_costmap, 1));
	}




	#[test]
	fn shuffle_works() {
		use rand::seq::SliceRandom;

		let mut deck = Deck::new(1);
		let cid: u8 = rand::random();
		deck.push(cid.into());	
		let cid: u8 = rand::random();
		deck.push(cid.into());	
		let cid: u8 = rand::random();
		deck.push(cid.into());	
		deck.shuffle();
		println!("{:?}", deck.rest_decks);
	}

	#[test]
	fn csv_io_works() {
		

	}


	#[ignore]
	// #[test]
	fn deck_shuffle_works() {

	}



}