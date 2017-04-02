use fx32::Fx32;
use fx32::Fx32Small;
use ray2::Ray2;
use vec2::Vec2;

pub struct Basis2 {
	pub x: Vec2 <Fx32Small>,
	pub y: Vec2 <Fx32Small>,
}

impl Basis2 {
	// Constructs a basis to ray space
	// Such that the ray is the X axis
	pub fn new (ray: &Ray2) -> Basis2 {
		let basis_x_big = if ray.get_length () == 0 {
			ray.get_dir ()
		}
		else {
			ray.get_dir () / ray.get_length ()
		};
		
		Basis2 {
			x: basis_x_big.to_small (),
			y: basis_x_big.cross ().to_small (),
		}
	}
	
	pub fn to_space (&self, v: &Vec2 <Fx32>) -> Vec2 <Fx32> {
		Vec2::<Fx32> {
			x: *v * self.x,
			y: *v * self.y,
		}
	}
	
	pub fn from_space (&self, v: &Vec2 <Fx32>) -> Vec2 <Fx32> {
		Vec2::<Fx32> {
			x: (v.x * self.x.x) + (v.y * self.y.x),
			y: (v.x * self.x.y) + (v.y * self.y.y),
		}
	}
}
