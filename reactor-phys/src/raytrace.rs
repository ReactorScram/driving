use circle::Circle;
use fx32::Fx32;
use fx32::Fx32Small;
use ray2::Ray2;
use vec2::Vec2;

extern crate svg;

use self::svg::Document;
use self::svg::node::element::Path;
use self::svg::node::element::path::Data;

#[derive (Clone, Copy)]
pub enum Ray2TraceResult {
	Hit (Fx32, Vec2 <Fx32>, Vec2 <Fx32Small>),
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

pub fn test_ray_trace () {
	let mut document = Document::new()
			.set("viewBox", (0, 0, 512, 512));
		
		let scale = 1;
		//let scale_fx = Fx32::from_int (scale);
		
		let obstacle = vec! [
		Circle {
			center: Vec2 {x: Fx32::from_q (256, scale), y: Fx32::from_q (501, scale)},
			radius: Fx32::from_q (50, scale),
		},
		Circle {
			center: Vec2 {x: Fx32::from_q (0, scale), y: Fx32::from_q (512 - 512, scale)},
			radius: Fx32::from_q (40, scale),
		},
		Circle {
			center: Vec2 {x: Fx32::from_q (404, scale), y: Fx32::from_q (512 - 41, scale)},
			radius: Fx32::from_q (60, scale),
		},
		];
		
		let mut num_bounces = 0;
		
		let beam_radius = 55;
		let beam_center = 256;
		
		let inv_dt = 64;
		
		for x in beam_center - beam_radius..beam_center + beam_radius {
			let mut particle = Ray2 {
				start: Vec2 {
					x: Fx32::from_q (x, scale),
					y: Fx32::from_q (0, scale)
				},
				dir: Vec2 {
					x: Fx32::from_q (0, scale),
					y: Fx32::from_q (inv_dt, scale),
				},
			};
			
			let mut data = Data::new().move_to(((particle.start.x).to_f64 (), (particle.start.y).to_f64 ()));
			
			for step in 0..1000 {
				let trace_result = obstacle.iter ().map(|obstacle: &Circle| ray_trace_circle_2 (&particle, obstacle)).fold ( Ray2TraceResult::Miss, fold_closer_result);
				
				let dt = Fx32::from_q (1, inv_dt).to_small ();
				
				match trace_result {
					Ray2TraceResult::Miss => {
						particle.start = particle.start + (particle.dir * dt);
					},
					Ray2TraceResult::Hit (t, ccd_pos, normal) => {
						particle.start = ccd_pos;
						particle.dir = particle.dir.reflect (normal);
						//particle.dir = normal;
						num_bounces += 1;
					},
				};
				
				data = data.line_to(((particle.start.x).to_f64 (), (particle.start.y).to_f64 ()));
			}
			
			//data = data.close ();
			
			let path = Path::new()
				.set("fill", "none")
				.set("stroke", "black")
				.set("stroke-width", 0.5)
				.set("d", data);
			
			document = document.add(path);
		}
		
		println! ("num_bounces: {}", num_bounces);
		
		svg::save("image.svg", &document).unwrap();
}

pub fn ray_trace_circle_2 (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let ray_length = ray.dir.length ();
	
	let basis_x_big = ray.dir / ray_length;
	
	let basis_x = basis_x_big.to_small ();
	let basis_y = basis_x_big.cross ().to_small ();
	
	let to_circle = circle.center - ray.start;
	
	let center_in_ray_space = Vec2::<Fx32> {
		x: to_circle * basis_x,
		y: to_circle * basis_y,
	};
	
	if center_in_ray_space.x < 0 {
		//return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.x > ray_length + circle.radius {
		//return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.y.abs () >= circle.radius {
		// Prevents a negative root later on
		return Ray2TraceResult::Miss;
	}
	
	let ray_space_x = center_in_ray_space.x - (circle.radius.square () - center_in_ray_space.y.square ()).sqrt_64 ();
	
	if ray_space_x > ray_length {
		//return Ray2TraceResult::Miss;
	}
	
	let t = ray_space_x / ray_length;
	let ccd_pos = ray.start + ray.dir * t;
	
	if t >= 0 && t <= 1 {
		return Ray2TraceResult::Hit (
			t,
			ccd_pos,
			((ccd_pos - circle.center) / circle.radius).to_small (),
		);
	}
	else {
		return Ray2TraceResult::Miss;
	}
}
