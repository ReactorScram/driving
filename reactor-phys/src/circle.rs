use fx32::Fx32;
use vec2::Vec2;

pub struct Circle {
	pub center: Vec2 <Fx32>,
	pub radius: Fx32,
}

impl Circle {
	pub fn signed_distance (&self, p: &Vec2 <Fx32>) -> Fx32 {
		(*p - self.center).length () - self.radius
	}
}
