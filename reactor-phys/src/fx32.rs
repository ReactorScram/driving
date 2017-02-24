// 16.16 fixed point

pub struct Fx32 {
	pub x: i32,
}

impl Fx32 {
	pub fn new (x: i32) -> Fx32 {
		Fx32 {
			x: x,
		}
	}
	
	pub fn add (a: &Fx32, b: &Fx32) -> Fx32 {
		Fx32 {
			x: a.x + b.x,
		}
	}
}
