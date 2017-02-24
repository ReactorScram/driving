// 16.16 fixed point

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;

#[derive (Clone, Copy, Eq, PartialEq)]
pub struct Fx32 {
	pub x: i32,
}

impl Fx32 {
	pub fn new (x: i32) -> Fx32 {
		Fx32 {
			x: x,
		}
	}
	
	pub fn from_float (x: f32) -> Fx32 {
		// TODO: Magic number
		Fx32::new ((x * 65536.0f32) as i32)
	}
	
	// More precise and automatic but requires a branch
	pub fn mul_precise (self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		if a.abs () > b.abs () {
			Fx32::new ((a / 256) * (b) / 256)
		}
		else {
			Fx32::new ((a) * (b / 256) / 256)
		}
	}
	
	// For multiplying two numbers <= 1.0 such as
	// color mixing or dotting unit vectors
	pub fn mul_small (self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		Fx32::new (((a / 2) * (b / 2)) / 16384)
	}
	
	/*
	I think this is the canonical way to do it,
	but it does require i64. Will need performance testing
	on the Pandora.
	*/
	pub fn mul_64 (self, o: Fx32) -> Fx32 {
		let c = self.x as i64 * o.x as i64;
		Fx32::new ((c / 65536) as i32)
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

impl Mul <Fx32> for Fx32 {
	type Output = Fx32;
	
	fn mul (self, o: Fx32) -> Fx32 {
		/*
		let c be the fixed point 'factor'
		a = 1.0
		b = 2.0
		c = 65536
		
		(a * c) * (b * c) ---> (a * b * c)
		
		We must end up dividing by c
		We can do this by dividing both a and b by root c
		*/
		
		Fx32::new ((self.x / 256) * (o.x / 256))
	}
}
