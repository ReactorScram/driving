// 32-bit fixed point. Haven't decided on the fractional bits yet

extern crate int_traits;

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use self::int_traits::IntTraits;

type Int = i32;
type DoubleInt = i64;

// Adjust this to balance between range and granularity
// 6 --> 20.12
// 8 --> 16.16
// 10 --> 12.20
pub const HALF_FRACTIONAL_BITS: Int = 8;

pub const FRACTIONAL_BITS: Int = HALF_FRACTIONAL_BITS * 2;
pub const DENOMINATOR: Int = 1 << FRACTIONAL_BITS;

#[derive (Clone, Copy, Eq, PartialEq)]
pub struct Fx32 {
	pub x: Int,
}

impl Debug for Fx32 {
    fn fmt(&self, f: &mut Formatter) -> Result {
		write! (f, "Fx32 {{ {} }}", self.to_f64 ())
    }
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
		Fx32::new ((((num as DoubleInt) << FRACTIONAL_BITS) / den as DoubleInt) as Int)
	}
	
	pub fn from_int (x: Int) -> Fx32 {
		Fx32::from_q (x, 1)
	}
	
	pub fn to_f64 (&self) -> f64 {
		self.x as f64 / DENOMINATOR as f64
	}
	
	pub fn to_i32 (&self) -> i32 {
		self.x >> FRACTIONAL_BITS
	}
	
	pub fn to_small (self) -> Fx32Small {
		assert! (self.abs () < 2);
		Fx32Small {
			x: self,
		}
	}
	
	pub fn abs (&self) -> Fx32 {
		Fx32::new (self.x.abs ())
	}
	
	// More precise and automatic but requires a branch
	pub fn mul_precise (self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		if a.abs () > b.abs () {
			Fx32::new ((a >> HALF_FRACTIONAL_BITS) * (b) >> HALF_FRACTIONAL_BITS)
		}
		else {
			Fx32::new ((a) * (b >> HALF_FRACTIONAL_BITS) >> HALF_FRACTIONAL_BITS)
		}
	}
	
	// For multiplying two numbers <= 1.0 such as
	// color mixing or dotting unit vectors
	pub fn mul_small (&self, o: Fx32) -> Fx32 {
		let a = self.x;
		let b = o.x;
		
		Fx32::new (((a / 2) * (b / 2)) >> (FRACTIONAL_BITS - 2))
	}
	
	pub fn mul_big (&self, o: Fx32) -> Fx32 {
		/*
		let c be the fixed point 'factor'
		a = 1.0
		b = 2.0
		c = 65536
		
		(a * c) * (b * c) ---> (a * b * c)
		
		We must end up dividing by c
		We can do this by dividing both a and b by root c
		*/
		
		Fx32::new ((self.x >> HALF_FRACTIONAL_BITS) * (o.x >> HALF_FRACTIONAL_BITS))
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
		Fx32::new ((c >> FRACTIONAL_BITS as DoubleInt) as Int)
	}
	
	pub fn div_32 (self, o: Fx32) -> Fx32 {
		Fx32::new ((self.x / (o.x >> HALF_FRACTIONAL_BITS)) << HALF_FRACTIONAL_BITS)
	}
	
	pub fn div_64 (self, o: Fx32) -> Fx32 {
		let a2 = (self.x as DoubleInt) << FRACTIONAL_BITS as DoubleInt;
		Fx32::new ((a2 / o.x as DoubleInt) as Int)
	}
	
	pub fn square (self) -> Fx32 {
		let a = self.x >> HALF_FRACTIONAL_BITS;
		Fx32 { x: a * a }
	}
	
	pub fn square_64 (self) -> Fx32 {
		Fx32::mul_64 (self, self)
	}
	
	pub fn sqrt_64 (self) -> Fx32 {
		Fx32 {
			x: ((self.x as DoubleInt) << FRACTIONAL_BITS).sqrt () as Int,
		}
	}
	
	pub fn sqrt (self) -> Fx32 {
		Fx32 { x: self.x.sqrt () << HALF_FRACTIONAL_BITS }
	}
}
/*
impl fmt::Debug for Fx32 {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "Fx32 ({})", self.to_f64 ())
    }
}
*/
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
		Fx32::mul_big (&self, o)
	}
}

impl Div <Fx32> for Fx32 {
	type Output = Fx32;
	
	fn div (self, o: Fx32) -> Fx32 {
		Fx32::div_32 (self, o)
	}
}

impl PartialOrd for Fx32 {
	fn partial_cmp (&self, o: &Fx32) -> Option <Ordering> {
		self.x.partial_cmp (&o.x)
	}
}

impl PartialEq <i32> for Fx32 {
	fn eq (&self, o: &i32) -> bool {
		self.x == Fx32::from_int (*o).x
	}
}

impl PartialOrd <i32> for Fx32 {
	fn partial_cmp (&self, o: &i32) -> Option <Ordering> {
		self.x.partial_cmp (&Fx32::from_int (*o).x)
	}
}
/*
A specialized fixed-point number with the same fractional bits,
but compile-time hints that it should fall in the range (-2.0, +2.0)
and asserts.
*/
#[derive (Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fx32Small {
	pub x: Fx32,
}

impl From <Fx32> for Fx32Small {
	fn from (o: Fx32) -> Fx32Small {
		o.to_small ()
	}
}

impl From <Fx32Small> for Fx32 {
	fn from (o: Fx32Small) -> Fx32 {
		o.x
	}
}

impl Fx32Small {
	pub fn mul_by_big (self, o: Fx32) -> Fx32 {
		// Assert for headroom on the big number
		assert! (o.abs () < 1 << (FRACTIONAL_BITS - 2));
		
		Fx32 {
			x: ((o.x >> (FRACTIONAL_BITS - 2)) * (self.x.x >> 2))
		}
	}
}

impl Mul <Fx32Small> for Fx32 {
	type Output = Fx32;
	
	fn mul (self, o: Fx32Small) -> Fx32 {
		o.mul_by_big (self)
		//o.x.mul_64 (self)
	}
}

impl Mul <Fx32> for Fx32Small {
	type Output = Fx32;
	
	fn mul (self, o: Fx32) -> Fx32 {
		self.mul_by_big (o)
		//self.x.mul_64 (o)
	}
}
