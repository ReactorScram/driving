use circle::Circle;
use fx32::Fx32;
use fx32::Fx32Small;
use vec2::Vec2;
use raytrace::Ray2TraceResult;

/*
In order to produce polycapsule shapes, we construct an arc which
is a truncated circle.

2     1
O-----O
|
|
|
O 3

For a 3-point polycapsule, each end will have a semicircle arc, and the 
arc in the middle (2) will be reduced so it only covers the convex side of
the joint. (The upper-left quadrant in this ascii diagram)
*/

// TODO: Allow degenerate Arcs to be Circles
#[derive (Clone, Copy, Debug)]
pub struct Arc {
	pub circle: Circle,
	pub rejected_normals: [Vec2 <Fx32Small>; 2],
}

impl Arc {
	pub fn new1 (circle: &Circle, neighbor: Vec2 <Fx32>) -> Arc {
		Arc::new2 (&circle, neighbor, neighbor)
	}
	
	// Oh won't you be my neighbor
	pub fn new2 (circle: &Circle, neighbor0: Vec2 <Fx32>, neighbor1: Vec2 <Fx32>) -> Arc 
	{
		let to_0 = (neighbor0 - circle.center) * Fx32::from_q (1, 256);
		let to_1 = (neighbor1 - circle.center) * Fx32::from_q (1, 256);
		
		Arc {
			circle: *circle,
			rejected_normals: [
				to_0.normalized (),
				to_1.normalized (),
			],
		}
	}
	
	pub fn filter_normal (&self, v: Vec2 <Fx32Small>) -> bool {
		if v * self.rejected_normals [0] > 0 {
			return false;
		}
		else if v * self.rejected_normals [1] > 0 {
			return false;
		}
		else {
			return true;
		}
	}
	
	pub fn filter_collision (&self, input: Ray2TraceResult) -> Ray2TraceResult 
	{
		match input {
			Ray2TraceResult::Miss => return Ray2TraceResult::Miss,
			Ray2TraceResult::Pop (__, normal) => {
				if self.filter_normal (normal) {
					return input;
				}
				else {
					return Ray2TraceResult::Miss;
				}
			},
			Ray2TraceResult::Hit (_, __, normal) => {
				if self.filter_normal (normal) {
					return input;
				}
				else {
					return Ray2TraceResult::Miss;
				}
			},
		}
	}
}
