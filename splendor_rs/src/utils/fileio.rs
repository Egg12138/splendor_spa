use crate::app::Color;
// fileio.rs
use crate::cards::{ GemNumMap, Card};
use crate::nobles::Noble;
use std::collections::HashMap;
use std::io::Write;
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
			let arr_deserde: [u8; 8] = raw_deserde.unwrap_or([0,0,0,0,0,0,0,0]);
			let card: Card = Card::from_arr(arr_deserde)?;
			cards.push(card);
		}
		// assert_eq!(cards.len(), 90);
		Ok(cards)
	} else {
		Err(From::from("Failed to read Cards from csv"))
	}
}

pub fn read_into_noblespool<P>(path: P) -> Result<Vec<Noble>, Box<dyn Error>> 
where P: AsRef<Path>,
{

	if let Ok(mut rdr) = csv::Reader::from_path(path) {
		let mut nobles: Vec<Noble> = Vec::new();
		let mut iter = rdr.deserialize();
		while let Some(raw_deserde) = iter.next() {
			let arr_deserde: [u8; 5] = raw_deserde.unwrap_or([0,0,0,0,0]);
			let noble: Noble = Noble::from_arr(arr_deserde);
			nobles.push(noble);
		}
		// assert_eq!(nobles.len(), 10);
		Ok(nobles)
	} else {
		Err(From::from("Failed to read Nobles from csv"))
	}
} 


#[deprecated]
pub fn noble_print(nobles: Vec<Noble>, mode: usize)  {

	use std::io::stdout;
	let stdout = stdout();
	let mut handle = stdout.lock();
	match mode {
		1 => { 
		for noble in nobles	{
			println!("{noble}");
			}
		}, 
		_ => {

		for noble in nobles	{
			println!("{:?}", noble);
			}
		}
	}


}