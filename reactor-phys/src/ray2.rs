use fx32::Fx32;
use vec2::Vec2;

pub struct Ray2 {
	pub start: Vec2 <Fx32>,
	pub dir: Vec2 <Fx32>,
}

impl Ray2 {
	pub fn at (&self, t: Fx32) -> Vec2 <Fx32> {
		// I don't know C++ but I _really_ don't know Rust
		// LLVM be strong for me
		self.start + self.dir * t
	}
}
