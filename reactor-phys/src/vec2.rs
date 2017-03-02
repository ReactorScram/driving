use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use fx32::Fx32;
use fx32::Fx32Small;

#[derive (Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec2 <Real> {
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

// Real: From <<Real as Div<Real2>>::Output> + Div <Real2>

impl <Real: Into <Fx32> + From <Fx32> + From <Fx32Small> + Neg + Mul <Fx32> + Mul <Fx32Small> + Mul <Real> + From <<Real as Mul <Real>>::Output> + From <<Real as Div <Real>>::Output> + Div <Real> + Mul <Vec2 <Fx32Small>> + From <<Vec2 <Real> as Mul>::Output>> Vec2 <Real> where Vec2 <Real>: Mul, Fx32: From <Real>, Vec2 <Real>: Mul <Vec2 <Fx32Small>>, Fx32: From <<Vec2 <Real> as Mul <Vec2 <Fx32Small>>>::Output>, Real: Copy {
	pub fn length_sq (self) -> Real {
		Real::from (self * self)
	}
	
	pub fn length (self) -> Real {
		let length2: Fx32 = self.length_sq ().into ();
		Real::from (length2.sqrt ())
	}
	
	pub fn normalized (self) -> Vec2 <Fx32Small> {
		let v = self / self.length ();
		Vec2::<Fx32Small> {
			x: v.x.into ().to_small (),
			y: v.y.into ().to_small (),
		}
	}
	
	// Given 2D space where X is right, and Y is up, like a math graph,
	// Rotates 90 degrees anti-clockwise (positive)
	pub fn cross (&self) -> Vec2 <Real> {
		let old_y: Fx32 = self.y.into ();
		
		Vec2::<Real> {
			x: Real::from (-old_y),
			y: self.x,
		}
	}
	
	// o should be normalized
	pub fn reflect (self, n: Vec2 <Fx32Small>) -> Vec2 <Real> {
		let projection = Fx32::from (self * n);
		let double_proj = Fx32 { x: projection.x * 2 };
		let offset = Vec2::<Fx32> {
			x: n.x * double_proj,
			y: n.y * double_proj,
		};
		
		self - Vec2::<Real> {
			x: offset.x.into (),
			y: offset.y.into (),
		}
	}
	
	pub fn to_small (self) -> Vec2 <Fx32Small> {
		Vec2::<Fx32Small> {
			x: self.x.into ().to_small (),
			y: self.y.into ().to_small (),
		}
	}
}

impl From <Vec2 <Fx32Small>> for Vec2 <Fx32> {
	fn from (o: Vec2 <Fx32Small>) -> Vec2 <Fx32> {
		Vec2::<Fx32> {
			x: o.x.into (),
			y: o.y.into (),
		}
	}
}

impl <Real: Into <Fx32> + From <Fx32>> Add <Vec2 <Real>> for Vec2 <Real> {
	type Output = Vec2 <Real>;
	
	fn add (self, o: Vec2 <Real>) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from (self.x.into () + o.x.into ()),
			y: Real::from (self.y.into () + o.y.into ()),
		}
	}
}

impl <Real: Into <Fx32> + From <Fx32>> Sub <Vec2 <Real>> for Vec2 <Real> {
	type Output = Vec2 <Real>;
	
	fn sub (self, o: Vec2 <Real>) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from (self.x.into () - o.x.into ()),
			y: Real::from (self.y.into () - o.y.into ()),
		}
	}
}

impl <Real: Into <Fx32> + From <Fx32>> Neg for Vec2 <Real> {
	type Output = Vec2 <Real>;
	
	fn neg (self) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from (-self.x.into ()),
			y: Real::from (-self.y.into ()),
		}
	}
}

// Dot product
impl <Real: Mul <Real2> + Into <Fx32> + From <Fx32> + From <<Real as Mul <Real2>>::Output>, Real2> Mul <Vec2 <Real2>> for Vec2 <Real> where Fx32: From <<Real as Mul <Real2>>::Output> {
	type Output = Real;
	
	fn mul (self, o: Vec2 <Real2>) -> Real {
		let xx = Fx32::from (self.x * o.x);
		let yy = Fx32::from (self.y * o.y);
		
		Real::from (xx + yy)
	}
}

// Scalar product
impl <Real: From <<Real as Mul<Real2>>::Output> + Mul <Real2>, Real2: Into <Fx32>> Mul <Real2> for Vec2 <Real> where Real: Copy, Real2: Copy {
	type Output = Vec2 <Real>;
	
	fn mul (self, o: Real2) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from ((self.x * o).into ()),
			y: Real::from ((self.y * o).into ()),
		}
	}
}

impl <Real> Mul <Vec2 <Real>> for Fx32 where Vec2 <Real>: Mul <Fx32>, Vec2 <Real>: From <<Vec2 <Real> as Mul <Fx32>>::Output> {
	type Output = Vec2 <Real>;
	
	fn mul (self, o: Vec2 <Real>) -> Vec2 <Real> {
		Vec2::<Real>::from (o * self)
	}
}

impl <Real> Mul <Vec2 <Real>> for Fx32Small where Vec2 <Real>: Mul <Fx32Small>, Vec2 <Real>: From <<Vec2 <Real> as Mul <Fx32Small>>::Output> {
	type Output = Vec2 <Real>;
	
	fn mul (self, o: Vec2 <Real>) -> Vec2 <Real> {
		Vec2::<Real>::from (o * self)
	}
}

impl <Real: From <<Real as Div<Real2>>::Output> + Div <Real2>, Real2: Into <Fx32>> Div <Real2> for Vec2 <Real> where Real: Copy, Real2: Copy {
	type Output = Vec2 <Real>;
	
	fn div (self, o: Real2) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from ((self.x / o).into ()),
			y: Real::from ((self.y / o).into ()),
		}
	}
}
/*
impl <Real: Div <Real>> Div <Real> for Vec2 <Real> {
	type Output = Vec2 <Real>;
	
	fn div (self, o: Real) -> Vec2 <Real> {
		Vec2::<Real> {
			x: Real::from ((self.x / o).into ()),
			y: Real::from ((self.y / o).into ()),
		}
	}
}
*/
