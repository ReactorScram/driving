use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;

use fx32;
type Real = fx32::Fx32;


#[derive (Clone, Copy, Eq, PartialEq)]
pub struct Vec4 {
	pub x: Real,
	pub y: Real,
	pub z: Real,
	pub w: Real,
}

impl Vec4 {
	pub fn new_3 (x: Real) -> Vec4 {
		Vec4 {
			x: x,
			y: x,
			z: x,
			w: Real::from_q (0, 1),
		}
	}
}

impl Add <Vec4> for Vec4 {
	type Output = Vec4;
	
	fn add (self, o: Vec4) -> Vec4 {
		Vec4 {
			x: self.x + o.x,
			y: self.y + o.y,
			z: self.z + o.z,
			w: self.w + o.w,
		}
	}
}

impl Sub <Vec4> for Vec4 {
	type Output = Vec4;
	
	fn sub (self, o: Vec4) -> Vec4 {
		Vec4 {
			x: self.x - o.x,
			y: self.y - o.y,
			z: self.z - o.z,
			w: self.w - o.w,
		}
	}
}

impl Neg for Vec4 {
	type Output = Vec4;
	
	fn neg (self) -> Vec4 {
		Vec4 {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}

// Dot product
impl Mul <Vec4> for Vec4 {
	type Output = Real;
	
	fn mul (self, o: Vec4) -> Real {
		self.x * o.x +
		self.y * o.y +
		self.z * o.z +
		self.w * o.w
	}
}

impl Mul <Real> for Vec4 {
	type Output = Vec4;
	
	fn mul (self, o: Real) -> Vec4 {
		Vec4 {
			x: self.x * o,
			y: self.y * o,
			z: self.z * o,
			w: self.w * o,
		}
	}
}
