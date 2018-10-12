// using LL for ends of snake
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
// draw_block helper fn from draw
use draw::draw_block;
// color elements: r, g, b, opacity 
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// allows cloning of dir, removes err, PEq allows equivalencies
#[derive(Copy, Clone, PartialEq)]
// direction of snake
pub enum Direction {
	Up, 
	Down, 
	Left,
	Right,
}

// implement Direction
impl Direction {
	pub fn opposite(&self) -> Direction {
		match *self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

#[derive(Debug, Clone)]
struct Block {
	x: i32,
	y: i32,
}

pub struct Snake {
	// direction snake is travelling in
	direction: Direction,
	// body of snake LL of blocks
	body: LinkedList<Block>,
	// tail of snake
	tail: Option<Block>,
}

impl Snake {
	pub fn new(x: i32, y: i32) -> Snake {
		// create mutable body
		let mut body: LinkedList<Block> = LinkedList::new();

		body.push_back(Block {
			// sets default snake on start with len() of 3
				x: x + 2,
				y,
		});
		body.push_back(Block{
			// snake start out horizontal x & y coord
			x: x + 1,
			y,
		});

		body.push_back(Block{
			x, 
			y,
		});

		Snake {
			// snake will start pointing in right direction 
			direction: Direction::Right,
			body,
			tail: None,
		}
	}
	

	pub fn draw(&self, con: &Context, g: &mut G2d){
		// iterate over block in body (self snake)
		for block in &self.body {
			// renders out snake
			draw_block(SNAKE_COLOR, block.x, block.y, con, g);
		}
	}

	pub fn head_position(&self) -> (i32, i32){
		// self outputs tuple of i32s

		// front provides reference to front of list, or none if none, 
		// avoid explicit error handling with unwrap
		let head_block = self.body.front().unwrap();
		// output headblock
		(head_block.x, head_block.y)
	}
	// take in mutable snake reference, 
	pub fn move_forward(&mut self, dir: Option<Direction>){
		// match on dir to get Option away
		match dir {
			// set snake dir to d
			Some(d) => self.direction = d,
			// else set to none
			None => (),
		}

		let (last_x, last_y): (i32, i32) = self.head_position();

		// match snake direction and bind to new_block
		let new_block = match self.direction {
			Direction::Up => Block {
				x: last_x,
				y: last_y - 1,
			},

			Direction::Down => Block {
				x: last_x,
				y: last_y + 1,
			},

			Direction::Left => Block {
				x: last_x - 1,
				y: last_y,
			},

			Direction::Right => Block {
				x: last_x + 1,
				y: last_y,
			},
		};
		// removing last block and adding new front
		self.body.push_front(new_block);
		// pop back pops off back part of LL + error handling unwrap
		let removed_block = self.body.pop_back().unwrap();
		self.tail = Some(removed_block);
	}

	pub fn head_direction(&self) -> Direction {
		self.direction
	}

	pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32){
		let (head_x, head_y): (i32, i32) = self.head_position();

		let mut moving_dir = self.direction;
		match dir {
			Some(d) => moving_dir = d,
			None => {}
		}

		// helps in finding head of snake
		match moving_dir {
			Direction::Up => (head_x, head_y - 1),
			Direction::Down => (head_x, head_y + 1),
			Direction::Left => (head_x - 1, head_y),
			Direction::Right => (head_x + 1, head_y), 
		}
	}

	pub fn restore_tail(&mut self){

		let blk = self.tail.clone().unwrap();
		// push cloned tail to back of body
		self.body.push_back(blk);
	}

	pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
		let mut i = 0;

		for block in &self.body {
			// check to see if any part of snake is overlapping with any part of its body
			if x == block.x && y == block.y{
				return true;
			}
			// otherwise increment
			i += 1;
			// check if i == len() of snake body - 1 // check if snake is overpassing tail
			if i == self.body.len() - 1 {
				// if so break;
				break;
			}
		}

		return false;
	}

}