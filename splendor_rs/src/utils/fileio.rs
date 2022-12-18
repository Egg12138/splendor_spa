// fileio.rs
use crate::cards::{Card, CostMap};

use std::path::Path;
use csv;

use std::error::Error;


/// 如果一次只读一个，实际上会很慢，所以我们直接得到全部的信息
pub fn read_into_cardspool<P>(path: P) -> Result<Vec<Card>, Box<dyn Error>> 
where P: AsRef<Path>
{
	if let Ok(mut rdr) = csv::Reader::from_path(path) {
		let mut cards: Vec<Card> = Vec::new();
		// for record_wrap in rdr.records() {
		// 	let record_raw = record_wrap?;
		// 	let arr = record_raw.as_slice();	
		// } 
		let mut iter = rdr.deserialize();
		while let Some(raw_deserde) = iter.next()	{
			let deserde: [u8; 8] = raw_deserde?;
			println!("{:?}", deserde);
		}
		Ok(cards)
	} else {
		Err(From::from("Failed to read csv"))
	}



}