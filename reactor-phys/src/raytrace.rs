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
	Pop (Vec2 <Fx32>, Vec2 <Fx32Small>),
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
				},
				Ray2TraceResult::Pop (..) => {
					return b;
				}
			}
		},
		Ray2TraceResult::Pop (..) => {
			return a;
		},
	}
}

pub fn write_vec2 <T> (writer: &mut T, v: &Vec2 <Fx32>) where T: Write {
	write! (writer, "v {} 0 {}\n", v.x.to_f64 () / 256.0, v.y.to_f64 () / 256.0).unwrap ();
}

pub fn apply_dt (ray: &Ray2, dt: Fx32Small) -> Ray2 {
	Ray2 {
		start: ray.start,
		dir: ray.dir * dt,
	}
}

pub fn test_ray_trace (filename: &str, offset: Fx32) -> Result <(), Error> {
	let scale = 1;
	//let scale_fx = Fx32::from_int (scale);
	
	let obstacle = vec! [
	Circle {
		center: Vec2 {x: Fx32::from_q (40, scale), y: Fx32::from_q (470 - 34, scale)},
		radius: Fx32::from_q (20, scale),
	},
	Circle {
		center: Vec2 {x: Fx32::from_q (256, scale), y: Fx32::from_q (512, scale)},
		radius: Fx32::from_q (20, scale),
	},
	Circle {
		center: Vec2 {x: Fx32::from_q (480, scale), y: Fx32::from_q (460, scale)},
		radius: Fx32::from_q (20, scale),
	},
	];
	
	let lines = vec! [
	WideLine {
		start: obstacle [0].center,
		end: obstacle [1].center,
		radius: Fx32::from_q (20, scale),
	},
	WideLine {
		start: obstacle [1].center,
		end: obstacle [2].center,
		radius: Fx32::from_q (20, scale),
	},
	];
	
	let mut num_bounces = 0;
	let mut num_pops = 0;
	let mut num_ticks = 0;
	
	let inv_dt = 1;
	let gravity = Vec2::<Fx32> {
		x: Fx32::from_q (0, 1),
		y: Fx32::from_q (2, 1),
	};
	
	let obj_file = try!(File::create(filename));
	let mut writer = BufWriter::new (obj_file);
	
	let mut vertex_i = 1;
	let mut polyline_start = vertex_i;
	
	for x in 0..32 {
		let x = x * 16;
		let mut particle = Ray2 {
			start: Vec2 {
				x: Fx32::from_q (x * 2, scale * 2) + offset,
				y: Fx32::from_q (0, scale)
			},
			dir: Vec2 {
				x: Fx32::from_q (0, scale),
				y: Fx32::from_q (1, scale),
			},
		};
		
		write_vec2 (&mut writer, &particle.start);
		vertex_i += 1;
		
		let dt = Fx32::from_q (1, inv_dt).to_small ();
		
		for tick in 0..500 {
			let trace_result = {
				let point_results = obstacle.iter ().map (|obstacle| ray_trace_circle_2 (&apply_dt (&particle, dt), obstacle));
				
				let line_results = lines.iter ().map (|line| ray_trace_line (&apply_dt (&particle, dt), line)); 
				
				point_results.chain (line_results).fold ( Ray2TraceResult::Miss, fold_closer_result)
			};
			
			match trace_result {
				Ray2TraceResult::Miss => {
					//let air_drag = Fx32 { x: particle.dir.length_sq ().x / -16384 };
					//let air_force = particle.dir * air_drag;
					let new_dir = particle.dir + gravity * dt;// + Vec2::<Fx32>::from (air_force * dt);
					/*
					let sum_dir = particle.dir + new_dir;
					let average_dir = Vec2::<Fx32> {
						x: Fx32 { x: sum_dir.x.x / 2 },
						y: Fx32 { x: sum_dir.y.x / 2 },
					};
					*/
					particle.start = particle.start + (particle.dir * dt);
					particle.dir = new_dir;
				},
				Ray2TraceResult::Pop (ccd_pos, normal) => {
					println! ("{}: Pop from {:?} to {:?}", tick, particle.start, ccd_pos);
					
					let reflected_dir = particle.dir.reflect (normal);
					
					let new_dir = reflected_dir + gravity * dt;
					/*
					let sum_dir = reflected_dir + new_dir;
					let average_dir = Vec2::<Fx32> {
						x: Fx32 { x: sum_dir.x.x / 2 },
						y: Fx32 { x: sum_dir.y.x / 2 },
					};
					*/
					particle.start = ccd_pos;// + (average_dir * dt);
					particle.dir = new_dir;
					
					num_pops += 1;
				},
				Ray2TraceResult::Hit (_, ccd_pos, normal) => {
					println! ("{}: Hit from {:?} to {:?}", tick, particle.start, ccd_pos);
					
					println! ("Incoming vel {:?}", particle.dir);
					
					particle.start = ccd_pos;
					if particle.dir * normal < 0 {
						particle.dir = particle.dir.reflect (normal) * Fx32::from_q (768, 1024);
					}
					
					println! ("Outgoing vel {:?}", particle.dir);
					
					//particle.dir = normal;
					num_bounces += 1;
				},
			};
			
			write_vec2 (&mut writer, &particle.start);
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
	println! ("num_pops: {}", num_pops);
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
	
	pub fn from_space (&self, v: &Vec2 <Fx32>) -> Vec2 <Fx32> {
		Vec2::<Fx32> {
			x: (v.x * self.x.x) + (v.y * self.y.x),
			y: (v.x * self.x.y) + (v.y * self.y.y),
		}
	}
}

pub fn ray_trace_line (ray: &Ray2, line: &WideLine) -> Ray2TraceResult {
	// Line has zero length and so zero collision area
	// If we run through the math we may get a 0 / 0 result
	// so just drop it now
	assert! (line.start != line.end);
	
	let ray_length_sq = ray.dir.length_sq ();
	let ray_length = ray_length_sq.sqrt ();
	
	let margin = Fx32::from_q (0, 256);
	
	let line_tangent: Vec2 <Fx32> = line.end - line.start;
	let line_tangent = Vec2::<Fx32> {
		x: line_tangent.x * Fx32::from_q (1, 256),
		y: line_tangent.y * Fx32::from_q (1, 256),
	};
	let line_normal = line_tangent.cross ().normalized ();
	
	let sdf: Fx32 = (ray.start - line.start) * line_normal;
	
	// Flip the normal so it's towards the ray
	// This will allow us to extrude the correct side
	let line_normal = if sdf > 0 {
		line_normal
	}
	else {
		-line_normal
	};
	
	let big_normal: Vec2 <Fx32> = line_normal.into ();
	
	let along = (ray.start - line.start) * line_tangent;
	let within_bounds = along > 0 && along < (line.end - line.start) * line_tangent;
	
	let starts_inside = sdf.abs () <= line.radius;
	let ends_inside = (ray.start + ray.dir - line.start) * line_normal <= line.radius;
	
	if (starts_inside || (ends_inside && ray_length == 0)) && within_bounds {
		// Ray has already started inside and we should push it out
		let towards = -Fx32::from (ray.dir * line_normal);
		
		let safe_point = ray.start + big_normal * (line.radius - sdf.abs () + margin);
		
		if towards > 0 {
			// Ray is moving deeper inside - Pop it out and reflect it
			return Ray2TraceResult::Pop (safe_point, line_normal);
		}
		else {
			// Ray is moving out - Pop it out but consume its timestep
			return Ray2TraceResult::Pop (safe_point, line_normal);
		}
	}
	
	if ray.dir * line_normal > 0 {
		return Ray2TraceResult::Miss;
	}
	
	let extrude_vector = (line.radius) * Vec2::<Fx32>::from (line_normal);
	// Extrude the line
	let line = WideLine {
		start: line.start + extrude_vector,
		end: line.end + extrude_vector,
		radius: Fx32::from_int (0),
	};
	
	if ray_length == 0 {
		//return Ray2TraceResult::Miss;
	}
	
	let basis = get_ray_basis (ray, ray_length);
	
	// Transform the line to ray space
	let line = WideLine {
		start: basis.to_space (&(line.start - ray.start)),
		end: basis.to_space (&(line.end - ray.start)),
		radius: line.radius,
	};
	
	// Line will have zero y difference resulting in a divide by zero
	if line.start.y == line.end.y {
		return Ray2TraceResult::Miss;
	}
	
	// Apply SAT
	// These are not causing the penetration bug
	if line.start.y > 0 && line.end.y > 0 {
		return Ray2TraceResult::Miss;
	}
	else if line.start.y < 0 && line.end.y < 0 {
		return Ray2TraceResult::Miss;
	}
	
	// If we get here, we know that the obstacle line crosses our ray somewhere
	
	// Note: Lines must not have 0 length
	let obstacle_t = (-line.start.y) / (line.end.y - line.start.y);
	
	assert! (obstacle_t >= 0);
	assert! (obstacle_t <= 1, format!("{:?}", ray_length));
	
	// lerp
	let crossing_x = line.start.x * (Fx32::from_int (1) - obstacle_t) + line.end.x * obstacle_t;
	
	if crossing_x <= 0 || crossing_x > ray_length + margin {
		return Ray2TraceResult::Miss;
	}
	
	// The line segments intersect
	let t = crossing_x / ray_length;
	
	let safe_point = ray.start + ray.dir * t /*+ big_normal * margin*/;
	
	return Ray2TraceResult::Hit (
		t,
		safe_point,
		line_normal,
	);
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
