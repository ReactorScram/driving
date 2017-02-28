use circle::Circle;
use fx32::Fx32;
use ray2::Ray2;
use vec2::Vec2;

#[derive (Clone, Copy)]
pub enum Ray2TraceResult {
	Hit (Fx32, Vec2, Vec2),
	Miss,
}

pub fn fold_closer_result (a: Ray2TraceResult, b: Ray2TraceResult) -> Ray2TraceResult {
	match a {
		Ray2TraceResult::Miss => {
			return b;
		},
		Ray2TraceResult::Hit (a_t, ..) => {
			match b {
				Ray2TraceResult::Miss => {
					return a;
				},
				Ray2TraceResult::Hit (b_t, ..) => {
					if a_t < b_t {
						return a;
					}
					else {
						return b;
					}
				}
			}
		},
	}
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
	
	let b = (ray.dir * toward_circle) * Fx32::from_int (-2);
	
	let c = toward_circle.length_sq () - circle.radius.square_64 ();
	
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
