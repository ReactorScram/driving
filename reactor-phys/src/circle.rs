use fx32::Fx32;
use vec2::Vec2;

pub struct Circle {
	center: Vec2,
	radius: Fx32,
}

impl Circle {
	pub fn signed_distance (&self, p: &Vec2) -> Fx32 {
		(*p - self.center).length () - self.radius
	}
}
