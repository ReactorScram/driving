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

pub fn ray_trace_circle_2 (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let ray_length = ray.dir.length ();
	
	let basis_x = ray.dir / ray_length;
	let basis_y = basis_x.cross ();
	
	let to_circle = circle.center - ray.start;
	
	let center_in_ray_space = Vec2 {
		x: to_circle * basis_x,
		y: to_circle * basis_y,
	};
	
	if center_in_ray_space.x < 0 {
		return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.x > ray_length + circle.radius {
		return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.y.abs () >= circle.radius {
		return Ray2TraceResult::Miss;
	}
	
	let ray_space_x = center_in_ray_space.x - (circle.radius.square_64 () - center_in_ray_space.y.square_64 ()).sqrt_64 ();
	
	if ray_space_x > ray_length {
		return Ray2TraceResult::Miss;
	}
	
	let t = ray_space_x / ray_length;
	let ccd_pos = ray.start + ray.dir * t;
	
	Ray2TraceResult::Hit (
		t,
		ccd_pos,
		(ccd_pos - circle.center) / circle.radius,
	)
}

pub fn ray_trace_circle (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let toward_circle = circle.center - ray.start;
	
	let ray_dot_circle = toward_circle * ray.dir;
	
	if ray_dot_circle < 0 {
		// Ray is heading out of the circle's space or is already out
		return Ray2TraceResult::Miss;
	}
	
	// The following is derived from the quadratic formula and my Lua code
	let a = ray.dir.length_sq ();
	
	let start_to_circle_sq = toward_circle.length_sq ();
	
	if start_to_circle_sq > (circle.radius + a.sqrt_64 ()).square_64 () {
		// The ray is so far away that it can not reach the circle
		return Ray2TraceResult::Miss;
	}
	
	if a == 0 {
		// Prevent a divide-by-zero
		return Ray2TraceResult::Miss;
	}
	
	// This part is different from the canonical quadratic formula
	// I negated it because it will save a negation later on
	// Then I also halved it
	let half_b = ray_dot_circle;
	
	let c = start_to_circle_sq - circle.radius.square_64 ();
	if c < 0 {
		// Ray is inside the circle, let it be
		return Ray2TraceResult::Miss;
	}
	
	let bb = half_b.square_64 ();
	let ac = a * c;
	
	if ac > bb {
		// Prevent imaginary numbers
		return Ray2TraceResult::Miss;
	}
	
	let quarter_determinant = bb - ac;
	
	let t = (half_b - quarter_determinant.sqrt_64 ()) / a;
	
	if t >= 0 && t <= 1 {
		let ccd_pos = ray.at (t);
		
		let normal = (ccd_pos - circle.center).normalized ();
		
		return Ray2TraceResult::Hit (t, ccd_pos, normal);
	}
	
	Ray2TraceResult::Miss
}
