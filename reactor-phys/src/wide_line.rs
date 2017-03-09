// Note: Not the same as a rect. Each endpoint should be capped with
// a circle

use fx32::Fx32;
use fx32::Fx32Small;
use vec2::Vec2;

#[derive (Clone, Copy, Debug)]
pub struct WideLine {
	pub start: Vec2 <Fx32>,
	pub end: Vec2 <Fx32>,
	pub radius: Fx32,
	pub line_tangent: Vec2 <Fx32Small>,
}

impl WideLine {
	pub fn new (start: Vec2 <Fx32>, end: Vec2 <Fx32>, radius: Fx32) -> WideLine 
	{
		let line_tangent: Vec2 <Fx32> = end - start;
		// TODO: Use 64-bit normalize since this is cached
		let line_tangent = Vec2::<Fx32> {
			x: line_tangent.x * Fx32::from_q (1, 256),
			y: line_tangent.y * Fx32::from_q (1, 256),
		}.normalized ();
		
		WideLine {
			start: start,
			end: end,
			radius: radius,
			line_tangent: line_tangent,
		}
	}
}
/*
impl Circle {
	pub fn signed_distance (&self, p: &Vec2 <Fx32>) -> Fx32 {
		(*p - self.center).length () - self.radius
	}
}
*/
