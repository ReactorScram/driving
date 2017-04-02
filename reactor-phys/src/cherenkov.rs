use circle::Circle;
use fx32::Fx32;
use polycapsule::PolyCapsule;
use raytrace::*;
use vec2::Vec2;

use std::mem::transmute;

pub struct CherenkovSim {
	pub obstacles: Vec <PolyCapsule>,
	pub player: Circle,
}

#[no_mangle]
pub extern fn cher_add (a: i32, b: i32) -> i32 {
	a + b
}

#[no_mangle]
pub extern fn cher_new (radius: f32) -> *mut CherenkovSim {
	let ctx = CherenkovSim {
		obstacles: vec! [],
		player: Circle {
			radius: Fx32::from_float (radius),
			center: Vec2::<Fx32> {
				x: Fx32::from_q (400, 1),
				y: Fx32::from_q (300, 1),
			},
		},
	};
	
	unsafe {
		transmute (Box::new (ctx))
	}
}

#[no_mangle]
pub extern fn cher_delete (opaque: *mut CherenkovSim) {
	let _context: Box <CherenkovSim> = unsafe {
		transmute (opaque)
	};
}



/*
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
					
					particle.start = ccd_pos;
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
	
	Ok (())
}
*/
