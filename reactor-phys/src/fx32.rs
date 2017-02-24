// 16.16 fixed point

use std::ops::Add;

#[derive (Clone, Copy)]
pub struct Fx32 {
	pub x: i32,
}

impl Fx32 {
	pub fn new (x: i32) -> Fx32 {
		Fx32 {
			x: x,
		}
	}
}

impl Add <Fx32> for Fx32 {
	type Output = Fx32;
	
	fn add (self, o: Fx32) -> Fx32 {
		Fx32 {
			x: self.x + o.x,
		}
	}
}
