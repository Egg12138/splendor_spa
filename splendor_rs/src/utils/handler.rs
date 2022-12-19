// tool functions

/// 提供一些公用的处理数据的函数
use crate::app::Color;
use crate::player::Player;
pub(crate) fn is_over(player_state: Player) -> bool {

		false

}

pub(crate) fn num_to_color(num: u8) -> Color {
	let color =  match num {
			1 => Color::Black,
			2 => Color::Blue,
			3 => Color::Green,
			4 => Color::Red,
			5 => Color::White,
			_ => Color::Gold,
		};
	color
} 

