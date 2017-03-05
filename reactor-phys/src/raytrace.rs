use arc::Arc;
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

pub struct PolyCapsule {
	pub arcs: Vec <Arc>,
	pub lines: Vec <WideLine>,
}

impl PolyCapsule {
	pub fn new (points: &[Vec2 <Fx32>], radius: Fx32) -> PolyCapsule
	{
		let count = points.len ();
		// A single circle is not a capsule
		assert! (count >= 2);
		
		let lines = {
			let mut lines = vec! [];
			
			lines.push (WideLine { start: points [0], end: points [1], radius: radius });
			
			for i in 1..count {
				lines.push (WideLine { start: points [i - 1], end: points [i], radius: radius });
			}
			
			lines
		};
		
		let arcs = {
			let mut arcs = vec! [];
			let circles: Vec <Circle> = points.iter ().map (|p| Circle { center: *p, radius: radius }).collect ();
			
			arcs.push (Arc::new1 (&circles [0], points [1]));
			
			for i in 1..count - 1 {
				arcs.push (Arc::new2 (&circles [i], points [i - 1], points [i + 1]));
			}
			
			arcs.push (Arc::new1 (&circles [count - 1], points [count - 2]));
			
			arcs
		};
		
		PolyCapsule {
			arcs: arcs,
			lines: lines,
		}
	}
	
	pub fn collect (capsules: &[PolyCapsule]) -> PolyCapsule {
		PolyCapsule {
			arcs: {
				let mut arcs = vec! [];
				
				for capsule in capsules.iter () {
					arcs.extend (capsule.arcs.clone ());
				}
				
				arcs
			},
			lines: {
				let mut lines = vec! [];
				
				for capsule in capsules.iter () {
					lines.extend (capsule.lines.clone ());
				}
				
				lines
			},
		}
	}
	
	pub fn affine <F> (&self, f: F) -> PolyCapsule where F: Fn (Vec2 <Fx32>) -> Vec2 <Fx32>
	{
		PolyCapsule {
			arcs: self.arcs.iter ().map (|a| Arc { circle: Circle { center: f (a.circle.center), radius: a.circle.radius }, rejected_normals: a.rejected_normals }).collect (),
			lines: self.lines.iter ().map (|l| WideLine { start: f (l.start), end: f (l.end), radius: l.radius }).collect (),
		}
	}
	
	pub fn translate (&self, offset: Vec2 <Fx32>) -> PolyCapsule {
		self.affine (|p| p + offset)
	}
}

pub fn test_ray_trace (filename: &str, offset: Fx32) -> Result <(), Error> {
	let scale = 1;
	//let scale_fx = Fx32::from_int (scale);
	
	let vec_from_q = |x, y, q| {
		Vec2 { x: Fx32::from_q (x, q), y: Fx32::from_q (y, q) }
	};
	
	let radius = Fx32::from_q (20, scale);
	
	let capsule = PolyCapsule::collect (&[
	PolyCapsule::new (&[
		vec_from_q (245, 240, scale),
		vec_from_q (255, 340, scale),
		vec_from_q (285, 340, scale),
		vec_from_q (295, 240, scale),
		vec_from_q (400, 340, scale),
		vec_from_q (450, 240, scale),
		vec_from_q (500, 340, scale),
		vec_from_q (600, 350, scale),
		vec_from_q (650, 330, scale),
	], radius),
	PolyCapsule::new (&[
		vec_from_q (210, 240, scale),
		vec_from_q (210, 340, scale),
	], radius),
	]).affine (|p| Vec2::<Fx32> {x: p.x, y: p.y} + vec_from_q (-200, 0, scale));
	
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
	
	for x in 0..128 {
		let x = x * 4;
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
				let dt_particle = apply_dt (&particle, dt);
				
				let point_results = capsule.arcs.iter ().map (|obstacle| ray_trace_arc (&dt_particle, obstacle));
				
				let line_results = capsule.lines.iter ().map (|line| ray_trace_line_2 (&dt_particle, line)); 
				
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
					
					let reflected_dir = particle.dir.reflect_res (normal, Fx32::from_q (512, 1024).to_small ());
					
					let new_dir = reflected_dir;// + gravity * dt;
					/*
					let sum_dir = reflected_dir + new_dir;
					let average_dir = Vec2::<Fx32> {
						x: Fx32 { x: sum_dir.x.x / 2 },
						y: Fx32 { x: sum_dir.y.x / 2 },
					};
					*/
					particle.start = ccd_pos;// + (average_dir * dt);
					if particle.dir * normal < 0 {
						particle.dir = new_dir;
					}
					
					num_pops += 1;
				},
				Ray2TraceResult::Hit (_, ccd_pos, normal) => {
					println! ("{}: Hit from {:?} to {:?}", tick, particle.start, ccd_pos);
					
					println! ("Incoming vel {:?}", particle.dir);
					
					particle.start = ccd_pos;
					if particle.dir * normal < 0 {
						particle.dir = particle.dir.reflect_res (normal, Fx32::from_q (512, 1024).to_small ());
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

pub fn ray_trace_line_2 (ray: &Ray2, line: &WideLine) -> Ray2TraceResult {
	let line_tangent: Vec2 <Fx32> = line.end - line.start;
	let huge_normal = line_tangent.cross ();
	let line_tangent = Vec2::<Fx32> {
		x: line_tangent.x * Fx32::from_q (1, 256),
		y: line_tangent.y * Fx32::from_q (1, 256),
	}.normalized ();
	let line_normal = Vec2::<Fx32Small> {
		x: (-Fx32::from (line_tangent.y)).to_small (),
		y: line_tangent.x,
	};
	
	let ray_end = ray.start + ray.dir;
	
	let start_along = (ray.start - line.start) * line_tangent;
	let end_along = (ray_end - line.start) * line_tangent;
	
	// TODO: Probably a way to optimize this into a series of inequalities
	let line_length = (line.end - line.start) * line_tangent;
	
	if start_along < 0 && end_along < 0 {
		return Ray2TraceResult::Miss;
	}
	if start_along > line_length && end_along > line_length {
		return Ray2TraceResult::Miss;
	}
	
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
	
	if ray.dir * big_normal > 0 {
		// Ray is leaving the half-plane, leave it be
		return Ray2TraceResult::Miss;
	}
	
	let start_distance = sdf.abs () - line.radius;
	let end_distance = (ray_end - line.start) * big_normal - line.radius;
	
	if start_distance < 0 {
		// Ray was already inside the plane, pop it out
		return Ray2TraceResult::Pop (
			ray.start + big_normal * (line.radius - sdf.abs ()),
			line_normal
		);
	}
	
	if end_distance >= 0 {
		// Ray will not reach the plane in this timestep, leave it be
		return Ray2TraceResult::Miss;
	}
	
	let t = (-start_distance) / (end_distance - start_distance);
	let ccd_pos = ray.start + ray.dir * t;
	
	let ccd_along = start_along * (Fx32::from_int (1) - t) + end_along * t;
	
	if ccd_along < 0 {
		return Ray2TraceResult::Miss;
	}
	if ccd_along > line_length {
		return Ray2TraceResult::Miss;
	}
	
	return Ray2TraceResult::Hit (t, ccd_pos, line_normal);
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
	let t = Fx32 { x: cmp::max (t.x, Fx32::from_int (0).x) };
	
	if t <= 1 {
		let ccd_pos = ray.start + ray.dir * t;
		let disp = ccd_pos - circle.center;
		let dist_sq = disp.length_sq ();
		let ccd_pos = if dist_sq < circle.radius.square () 
		{
			circle.center + disp * (circle.radius / dist_sq.sqrt ())
		}
		else {
			ccd_pos
		};
		
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

pub fn ray_trace_arc (ray: &Ray2, arc: &Arc) -> Ray2TraceResult {
	let circle_result = ray_trace_circle_2 (ray, &arc.circle);
	arc.filter_collision (circle_result)
}
