// 32-bit fixed point. Haven't decided on the fractional bits yet

extern crate int_traits;

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;
use self::int_traits::IntTraits;

type Int = i32;
type DoubleInt = i64;

// Adjust this to balance between range and granularity
// 6 --> 20.12
// 8 --> 16.16
// 10 --> 12.20
pub const HALF_FRACTIONAL_BITS: Int = 6;

pub const FRACTIONAL_BITS: Int = HALF_FRACTIONAL_BITS * 2;
pub const ROOT_DENOMINATOR: Int = 1 << HALF_FRACTIONAL_BITS;
pub const DENOMINATOR: Int = 1 << FRACTIONAL_BITS;

#[derive (Clone, Copy, Eq, PartialEq)]
pub struct Fx32 {
	pub x: Int,
}

impl Fx32 {
	pub fn new (x: Int) -> Fx32 {
		Fx32 {
			x: x,
		}
	}
	
	pub fn from_float (x: f32) -> Fx32 {
		Fx32::new ((x * DENOMINATOR as f32) as Int)
	}
	
	pub fn from_q (num: Int, den: Int) -> Fx32 {
		Fx32::new ((num * DENOMINATOR) / den)
	}
	
	pub fn from_int (x: Int) -> Fx32 {
		Fx32::from_q (x, 1)
	}
	
	// More precise and automatic but requires a branch
	pub fn mul_precise (self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		if a.abs () > b.abs () {
			Fx32::new ((a / ROOT_DENOMINATOR) * (b) / ROOT_DENOMINATOR)
		}
		else {
			Fx32::new ((a) * (b / ROOT_DENOMINATOR) / ROOT_DENOMINATOR)
		}
	}
	
	// For multiplying two numbers <= 1.0 such as
	// color mixing or dotting unit vectors
	pub fn mul_small (self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		Fx32::new (((a / 2) * (b / 2)) / (DENOMINATOR / 4))
	}
	
	/*
	I think this is the canonical way to do it,
	but it does require i64. Will need performance testing
	on the Pandora.
	
	I would recommend starting with this as a reference,
	and dropping precision where we can afford it based on
	regression testing.
	*/
	pub fn mul_64 (self, o: Fx32) -> Fx32 {
		let c = self.x as DoubleInt * o.x as DoubleInt;
		Fx32::new ((c / DENOMINATOR as DoubleInt) as Int)
	}
	
	pub fn square_64 (self) -> Fx32 {
		Fx32::mul_64 (self, self)
	}
	
	pub fn square_root_64 (self) -> Fx32 {
		Fx32 {
			x: (self.x as DoubleInt * DENOMINATOR as DoubleInt).sqrt () as Int,
		}
	}
}

impl Add <Fx32> for Fx32 {
	type Output = Fx32;
	
	fn add (self, o: Fx32) -> Fx32 {
		Fx32::new (self.x + o.x)
	}
}

// This is stupid
impl <'a, 'b> Add <&'a Fx32> for &'b Fx32 {
	type Output = Fx32;
	
	fn add (self, o: &'a Fx32) -> Fx32 {
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
		
		Fx32::new ((self.x / ROOT_DENOMINATOR) * (o.x / ROOT_DENOMINATOR))
	}
}
