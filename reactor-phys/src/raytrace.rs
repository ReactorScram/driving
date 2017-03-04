use circle::Circle;
use fx32::Fx32;
use fx32::Fx32Small;
use ray2::Ray2;
use vec2::Vec2;
use wide_line::WideLine;

use std::cmp;

use std::io::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

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

pub fn test_ray_trace () -> Result <(), Error> {
	let scale = 1;
	//let scale_fx = Fx32::from_int (scale);
	
	let obstacle = vec! [
	Circle {
		center: Vec2 {x: Fx32::from_q (133, scale), y: Fx32::from_q (512 - 34, scale)},
		radius: Fx32::from_q (20, scale),
	},
	Circle {
		center: Vec2 {x: Fx32::from_q (256, scale), y: Fx32::from_q (501, scale)},
		radius: Fx32::from_q (20, scale),
	},
	Circle {
		center: Vec2 {x: Fx32::from_q (404, scale), y: Fx32::from_q (512 - 41, scale)},
		radius: Fx32::from_q (20, scale),
	},
	];
	
	let lines = vec! [
	WideLine {
		start: obstacle [0].center,
		end: obstacle [1].center,
		radius: Fx32::from_q (20, scale),
	},
	];
	
	let mut num_bounces = 0;
	let mut num_ticks = 0;
	
	let inv_dt = 64;
	let gravity = Vec2::<Fx32> {
		x: Fx32::from_q (0, 1),
		y: Fx32::from_q (2, 1),
	};
	
	let obj_file = try!(File::create("lines.obj"));
	let mut writer = BufWriter::new (obj_file);
	
	let mut vertex_i = 1;
	let mut polyline_start = vertex_i;
	
	for x in 0..256 {
		let x = x * 2;
		let mut particle = Ray2 {
			start: Vec2 {
				x: Fx32::from_q (x * 2 + 1, scale * 2),
				y: Fx32::from_q (0, scale)
			},
			dir: Vec2 {
				x: Fx32::from_q (0, scale),
				y: Fx32::from_q (0, scale),
			},
		};
		
		try! (write! (writer, "v {} {} 0\n", particle.start.x.to_f64 (), particle.start.y.to_f64 ()));
		vertex_i += 1;
		
		for _ in 0..4000 {
			let trace_result = {
				let point_results = obstacle.iter ().map (|obstacle| ray_trace_circle_2 (&particle, obstacle));
				
				let line_results = lines.iter ().map (|line| ray_trace_line (&particle, line)); 
				
				point_results.chain (line_results).fold ( Ray2TraceResult::Miss, fold_closer_result)
			};
			
			let dt = Fx32::from_q (1, inv_dt).to_small ();
			
			match trace_result {
				Ray2TraceResult::Miss => {
					let air_drag = Fx32 { x: particle.dir.length_sq ().x / -16384 };
					let air_force = particle.dir * air_drag;
					let new_dir = particle.dir + gravity * dt + Vec2::<Fx32>::from (air_force * dt);
					let sum_dir = particle.dir + new_dir;
					let average_dir = Vec2::<Fx32> {
						x: Fx32 { x: sum_dir.x.x / 2 },
						y: Fx32 { x: sum_dir.y.x / 2 },
					};
					
					particle.start = particle.start + (average_dir * dt);
					particle.dir = new_dir;
				},
				Ray2TraceResult::Hit (_, ccd_pos, normal) => {
					particle.start = ccd_pos;
					particle.dir = particle.dir.reflect (normal) * Fx32::from_q (1023, 1024);
					//particle.dir = normal;
					num_bounces += 1;
				},
			};
			
			try! (write! (writer, "v {} {} 0\n", particle.start.x.to_f64 (), particle.start.y.to_f64 ()));
			vertex_i += 1;
			
			num_ticks += 1;
			
			if particle.start.y > 768 {
				break;
			}
		}
		
		for i in polyline_start..vertex_i - 1 {
			try! (write! (writer, "f {} {}\n", i, i + 1));
		}
		polyline_start = vertex_i
	}
	
	println! ("num_bounces: {}", num_bounces);
	println! ("num_ticks: {}", num_ticks);
	
	Ok (())
}

pub struct Basis2 {
	pub x: Vec2 <Fx32Small>,
	pub y: Vec2 <Fx32Small>,
}

// Constructs a basis to ray space
// Such that the ray is the X axis
pub fn get_ray_basis (ray: &Ray2, ray_length: Fx32) -> Basis2 {
	let basis_x_big = if ray_length == 0 {
		ray.dir
	}
	else {
		ray.dir / ray_length
	};
	
	Basis2 {
		x: basis_x_big.to_small (),
		y: basis_x_big.cross ().to_small (),
	}
}

impl Basis2 {
	pub fn to_space (&self, v: &Vec2 <Fx32>) -> Vec2 <Fx32> {
		Vec2::<Fx32> {
			x: *v * self.x,
			y: *v * self.y,
		}
	}
}

pub fn ray_trace_line (ray: &Ray2, line: &WideLine) -> Ray2TraceResult {
	Ray2TraceResult::Miss
}

pub fn ray_trace_circle_2 (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let ray_length = ray.dir.length ();
	let basis = get_ray_basis (ray, ray_length);
	
	let center_in_ray_space = basis.to_space (&(circle.center - ray.start));
	
	if center_in_ray_space.x < 0 {
		return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.x > ray_length + circle.radius {
		return Ray2TraceResult::Miss;
	}
	
	if center_in_ray_space.y.abs () >= circle.radius {
		// Prevents a negative root later on
		return Ray2TraceResult::Miss;
	}
	
	let ray_space_x = center_in_ray_space.x - (circle.radius.square () - center_in_ray_space.y.square ()).sqrt ();
	
	if ray_space_x > ray_length || ray_length == 0 {
		return Ray2TraceResult::Miss;
	}
	
	let t = ray_space_x / ray_length;
	let ccd_pos = ray.start + ray.dir * t;
	
	if t <= 1 {
		return Ray2TraceResult::Hit (
			Fx32 { x: cmp::max (t.x, Fx32::from_int (0).x) },
			ccd_pos,
			((ccd_pos - circle.center) / circle.radius).to_small (),
		);
	}
	else {
		return Ray2TraceResult::Miss;
	}
}
