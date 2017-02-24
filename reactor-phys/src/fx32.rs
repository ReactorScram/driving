// 16.16 fixed point

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

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
		Fx32::new (self.x + o.x)
	}
}

impl Sub <Fx32> for Fx32 {
	type Output = Fx32;
	
	fn sub (self, o: Fx32) -> Fx32 {
		Fx32::new (self.x - o.x)
	}
}

impl Neg for Fx32 {
	type Output = Fx32;
	
	fn neg (self) -> Fx32 {
		Fx32::new (-self.x)
	}
}
