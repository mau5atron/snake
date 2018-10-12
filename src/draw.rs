// import piston_window dep // definition of rectangle, 
// Context allows drawing of 2d content, G2d to draw
use piston_window::{rectangle, Context, G2d};
use piston_window::types::color;
// const ,type annotation, value of const
const BLOCK_SIZE: f64 = 25.0;

// pub export to program
pub fn to_coord(game_coord: i32) => f64 {
	// scale up by block size
	(game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d){
	let gui_x = to_coord(x);
	let gui_y = to_coord(y);

	rectangle(
		color, 
		// x val, y val, width, height
		[gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
		// context transform
		con.transform,
		// graphics buffer
		g,
	);
}