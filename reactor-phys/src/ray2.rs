use fx32::Fx32;
use vec2::Vec2;

pub struct Ray2 {
	pub start: Vec2 <Fx32>,
	dir: Vec2 <Fx32>,
	length: Fx32,
}

impl Ray2 {
	pub fn new (start: Vec2 <Fx32>, dir: Vec2 <Fx32>) -> Ray2 
	{
		Ray2 {
			start: start,
			dir: dir,
			length: dir.length (),
		}
	}
	
	pub fn at (&self, t: Fx32) -> Vec2 <Fx32> {
		// I don't know C++ but I _really_ don't know Rust
		// LLVM be strong for me
		self.start + self.dir * t
	}
	
	pub fn get_dir (&self) -> Vec2 <Fx32> {
		self.dir
	}
	
	pub fn get_length (&self) -> Fx32 {
		self.length
	}
}
