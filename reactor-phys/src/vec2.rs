use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;

use fx32;
type Real = fx32::Fx32;

#[derive (Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2 {
	pub x: Real,
	pub y: Real,
}
/*
impl fmt::Debug for Vec2 {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		write! (f, "Vec2 ({}, {})", self.x, self.y)
	}
}
*/

impl Vec2 {
	pub fn length_sq (&self) -> Real {
		self * self
	}
	
	pub fn length (&self) -> Real {
		self.length_sq ().sqrt_64 ()
	}
}

impl Add <Vec2> for Vec2 {
	type Output = Vec2;
	
	fn add (self, o: Vec2) -> Vec2 {
		Vec2 {
			x: self.x + o.x,
			y: self.y + o.y,
		}
	}
}

impl Sub <Vec2> for Vec2 {
	type Output = Vec2;
	
	fn sub (self, o: Vec2) -> Vec2 {
		Vec2 {
			x: self.x - o.x,
			y: self.y - o.y,
		}
	}
}

impl Neg for Vec2 {
	type Output = Vec2;
	
	fn neg (self) -> Vec2 {
		Vec2 {
			x: -self.x,
			y: -self.y,
		}
	}
}

// Dot product
impl <'a, 'b> Mul <&'a Vec2> for &'b Vec2 {
	type Output = Real;
	
	fn mul (self, o: &'a Vec2) -> Real {
		self.x * o.x +
		self.y * o.y
	}
}

impl <'a> Mul <Real> for &'a Vec2 {
	type Output = Vec2;
	
	fn mul (self, o: Real) -> Vec2 {
		Vec2 {
			x: self.x * o,
			y: self.y * o,
		}
	}
}
