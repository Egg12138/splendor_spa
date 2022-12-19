#![allow(dead_code)]
use crate::app::*;
use crate::utils;
use crate::cards::*;
use crate::nobles::*;
use crate::player::Player;


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

		let mut costs = GemNumMap::default();
		let new_color = Color::Blue;
		let new_black = Color::Black;
		costs.insert(new_color, 3);
		costs.insert(new_black, 2);
		assert_eq!(&mut costs, GemNumMap::default()
							.insert(Color::Blue, 3)
							.insert(Color::Black, 2)
				);
		assert_eq!(&mut costs.nonzero_costs(), GemNumMap::new()
							.insert(Color::Blue, 3)
							.insert(Color::Black, 2)
				);
	}


	#[test]
	fn cost_map_works() {
		let mut cost = GemNumMap::default(); 
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
		let costs = GemNumMap::default();
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
		let card = Card::from_arr([1, 1, 0, 0, 2, 1, 1, 1]).unwrap();
		let noble = Noble::demo();
		let cmp_costmap = GemNumMap::from_arr_ref(&[0u8, 0u8, 2u8, 1u8, 1u8]);
		assert_eq!(noble, Noble { requirement: cmp_costmap.clone() });	
		assert_eq!(card, Card::new(1, Color::Black, cmp_costmap, 1));
	}




	#[ignore]
	fn shuffle_works() {
	//IMPL 需要重新实现
	}

	#[test]
	fn from_csv2arr_into() {
		use utils::fileio::*;
		let cardscsv = "../cards.csv";
		let noblescsv = "../nobles.csv";
		// let noblescsv = "nobles.csv";
		let cards_pool = read_into_cardspool(cardscsv)
				.unwrap_or_default();

		assert_eq!(cards_pool.len(), 90);	
		let nobles_pool = read_into_noblespool(noblescsv)
				.unwrap_or_default();
		assert_eq!(nobles_pool.len(), 10);
		assert_eq!(cards_pool.get(3), Some(Card::from_arr([0,4,0,0,2,1,0,1]).unwrap()).as_ref());
		assert_eq!(cards_pool.get(14), Some(Card::new(1,Color::White,GemNumMap::from_arr_ref(&[0u8, 0u8,0u8,0u8,4u8]),1)).as_ref());
		assert_eq!(nobles_pool.get(9), Some(&Noble::from_arr_unwrap([3,3,0,0,3])));
		let mut map = GemNumMap::default();
		map.insert(Color::Black, 3)
			.insert(Color::Blue, 3)
			.insert(Color::Green, 0)
			.insert(Color::Red, 0)
			.insert(Color::White, 3);
		let map = map.clone();
		assert_eq!(nobles_pool.get(9)	, Some(&Noble { 
									requirement: map
								}));


	}

	#[test]
	fn from_csv2str() {
		use utils::fileio;
		let nobles = fileio::read_into_noblespool("../nobles.csv");
		fileio::noble_print(nobles.unwrap_or_default(), 2);
	}


	#[test]
	fn player_spend_works() {
		let mut p0 = Player::init(0);
		let mut p1 = Player::init(1);
		assert_eq!(p0.once_pick_max() * p1.once_pick_max(), 9);

	}

	#[test]
	fn player_picked_gems_works() {

		// 假设数量超过7来测试

	}

	#[ignore]
	// #[test]
	fn deck_shuffle_works() {

	}



}