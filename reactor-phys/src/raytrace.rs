use arc::Arc;
use basis::Basis2;
use circle::Circle;
use fx32::Fx32;
use fx32::Fx32Small;
use polycapsule::PolyCapsule;
use ray2::Ray2;
use raytraceresult::Ray2TraceResult;
use vec2::Vec2;
use wide_line::WideLine;

use std::cmp;

use std::io::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

pub fn write_vec2 <T> (writer: &mut T, v: &Vec2 <Fx32>, clock: Fx32) where T: Write {
	write! (writer, "v {} 0 {} {}\n", v.x.to_f64 (), v.y.to_f64 (), clock.to_f64 ()).unwrap ();
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
	let mut num_slips = 0;
	
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
		let mut particle = Ray2::new (
			Vec2 {
				x: Fx32::from_q (x * 2, scale * 2) + offset,
				y: Fx32::from_q (0, scale)
			},
			Vec2 {
				x: Fx32::from_q (0, scale),
				y: Fx32::from_q (1, scale),
			},
		);
		
		let mut clock = Fx32::from_int (0);
		
		write_vec2 (&mut writer, &particle.start, clock);
		vertex_i += 1;
		
		let dt = Fx32::from_q (1, inv_dt).to_small ();
		
		for tick in 0..200 {
			let mut remaining_dt = Fx32::from_int (1);
			
			particle = Ray2::new (particle.start, particle.get_dir () + gravity * remaining_dt);
			
			for subtick in 0..4 {
			let trace_result = {
				let dt_particle = particle.apply_dt (remaining_dt.to_small ());
				
				let point_results = capsule.arcs.iter ().map (|obstacle| ray_trace_arc (&dt_particle, obstacle));
				
				let line_results = capsule.lines.iter ().map (|line| ray_trace_line_2 (&dt_particle, line)); 
				
				point_results.chain (line_results).fold ( Ray2TraceResult::Miss, Ray2TraceResult::fold)
			};
			
			match trace_result {
				Ray2TraceResult::Miss => {
					particle.start = particle.start + (particle.get_dir () * remaining_dt);
					// Consume the entire remaining tick timestep
					clock = clock + Fx32::from (remaining_dt);
					remaining_dt = Fx32::from_int (0);
				},
				Ray2TraceResult::Pop (ccd_pos, normal) => {
					//println! ("{}: Pop from {:?} to {:?}", tick, particle.start, ccd_pos);
					
					let reflected_dir = particle.get_dir ().reflect_res (normal, Fx32::from_q (0, 1024).to_small ());
					
					let new_dir = reflected_dir;
					
					particle.start = ccd_pos;// + (average_dir * dt);
					if particle.get_dir () * normal < 0 {
						particle = Ray2::new (particle.start, new_dir);
					}
					
					//println! ("Vel. out: {:?}", particle.get_dir ());
					
					num_pops += 1;
					// Consume no time - This may lead to time dilation
					// for some objects if we run short of CPU
				},
				Ray2TraceResult::Hit (t, ccd_pos, normal) => {
					//println! ("{}: Hit from {:?} to {:?}", tick, particle.start, ccd_pos);
					
					//println! ("Incoming vel {:?}", particle.get_dir ());
					
					particle.start = ccd_pos;
					if particle.get_dir () * normal < 0 {
						particle = Ray2::new (particle.start, particle.get_dir ().reflect_res (normal, Fx32::from_q (512, 1024).to_small ()));
					}
					
					//println! ("Outgoing vel {:?}", particle.get_dir ());
					
					num_bounces += 1;
					// TODO: only works if dt == 1
					// Consume just the right portion of time
					let consumed_time = remaining_dt * Fx32::from (t);
					remaining_dt = remaining_dt - consumed_time;
					clock = clock + consumed_time;
				},
			};
			
			write_vec2 (&mut writer, &particle.start, clock);
			vertex_i += 1;
			num_ticks += 1;
			
			if remaining_dt <= Fx32::from_int (0) {
				break;
			}
			}
			
			if remaining_dt > Fx32::from_int (0) {
				num_slips += 1;
			}
			
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
	println! ("num_slips: {}", num_slips);
	
	Ok (())
}

pub fn ray_trace_line_2 (ray: &Ray2, line: &WideLine) -> Ray2TraceResult {
	// Quick AABB rejection
	if cmp::min (ray.start.x.x, ray.start.x.x + ray.get_dir ().x.x) > cmp::max (line.start.x.x + line.radius.x, line.end.x.x + line.radius.x) {
		return Ray2TraceResult::Miss;
	}
	if cmp::max (ray.start.x.x, ray.start.x.x + ray.get_dir ().x.x) < cmp::min (line.start.x.x - line.radius.x, line.end.x.x - line.radius.x) {
		return Ray2TraceResult::Miss;
	}
	
	let line_tangent = line.line_tangent;
	let line_normal = Vec2::<Fx32Small> {
		x: (-Fx32::from (line_tangent.y)).to_small (),
		y: line_tangent.x,
	};
	
	let ray_end = ray.start + ray.get_dir ();
	
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
	
	if ray.get_dir () * big_normal > 0 {
		// Ray is leaving the half-plane, leave it be
		return Ray2TraceResult::Miss;
	}
	
	let start_distance = sdf.abs () - line.radius;
	let end_distance = (ray_end - line.start) * big_normal - line.radius;
	
	if end_distance >= 0 {
		// Ray will not reach the plane in this timestep, leave it be
		return Ray2TraceResult::Miss;
	}
	
	if (end_distance - start_distance).x >> 8 == 0 {
		// Divide by zero
		return Ray2TraceResult::Miss;
	}
	
	let t = (-start_distance) / (end_distance - start_distance);
	let ccd_along = start_along * (Fx32::from_int (1) - t) + end_along * t;
	
	if ccd_along < 0 {
		return Ray2TraceResult::Miss;
	}
	if ccd_along > line_length {
		return Ray2TraceResult::Miss;
	}
	
	if start_distance < 0 {
		// Ray was already inside the plane, pop it out
		return Ray2TraceResult::Pop (
			ray.start + big_normal * (line.radius - sdf.abs ()),
			line_normal
		);
	}
	
	let ccd_pos = ray.start + ray.get_dir () * t;
	
	return Ray2TraceResult::Hit (t.to_small (), ccd_pos, line_normal);
}

pub fn ray_trace_circle_2 (ray: &Ray2, circle: &Circle) -> Ray2TraceResult {
	let ray_length = ray.get_length ();
	
	let diff = ray.start - circle.center;
	
	let max_diff = ray.get_length () + circle.radius + Fx32::from_q (3, 2);
	
	// Earlier rejection test
	if diff.x.abs () > max_diff || diff.y.abs () > max_diff {
		return Ray2TraceResult::Miss;
	}
	
	// Early rejection test
	if diff.length_sq () > max_diff.square ()
	{
		return Ray2TraceResult::Miss;
	}
	
	let basis = Basis2::new (ray);
	
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
	let t = Fx32 { x: cmp::max (t.x, Fx32::from_int (0).x) }.to_small ();
	
	if t <= 1 {
		let ccd_pos = ray.start + ray.get_dir () * t;
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
