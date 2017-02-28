use circle::Circle;
use fx32::Fx32;
use ray2::Ray2;
use std::iter::Iterator;
use vec2::Vec2;

pub enum Ray2TraceResult {
	Hit (Fx32, Vec2, Vec2),
	Miss,
}

pub fn closest_result (results: &mut Iterator <Item=Ray2TraceResult>) -> Ray2TraceResult {
	let mut best = Ray2TraceResult::Miss;
	
	for res in results {
		match res {
			Ray2TraceResult::Miss => {
				// Pass
			},
			Ray2TraceResult::Hit (res_t, ..) => {
				match best {
					Ray2TraceResult::Miss => {
						best = res;
					},
					Ray2TraceResult::Hit (best_t, ..) => {
						if res_t < best_t {
							best = res;
						}
					}
				}
			},
		};
	}
	
	best
}

pub fn ray_trace_circle (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let toward_circle = circle.center - ray.start;
	
	if toward_circle * ray.dir < Fx32::from_int (0) {
		// Ray is heading out of the circle's space or is already out
		return Ray2TraceResult::Miss;
	}
	
	// TODO: Handle ray starting inside circle
	
	// The following is derived from the quadratic formula and my Lua code
	let a = ray.dir.length_sq ();
	
	if a == Fx32::from_int (0) {
		return Ray2TraceResult::Miss;
	}
	
	let b = 
		Fx32::from_int (2) * (ray.start * ray.dir) -
		Fx32::from_int (2) * (circle.center * ray.dir);
	
	let c = circle.center.length_sq () + ray.start.length_sq ()
		- Fx32::from_int (2) * (circle.center * ray.start)
		- circle.radius.square_64 ();
	
	let determinant = b * b - Fx32::from_int (4) * a * c;
	if determinant < Fx32::from_int (0) {
		return Ray2TraceResult::Miss;
	}
	
	let t = (-b - determinant.sqrt_64 ()) / (Fx32::from_int (2) * a);
	
	if t >= Fx32::from_int (0) && t <= Fx32::from_int (1) {
		let ccd_pos = ray.at (t);
		
		let normal = (ccd_pos - circle.center).normalized ();
		
		return Ray2TraceResult::Hit (t, ccd_pos, normal);
	}
	
	Ray2TraceResult::Miss
}
