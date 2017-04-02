use circle::Circle;
use fx32::Fx32;
use polycapsule::PolyCapsule;
use raytrace::*;
use ray2::Ray2;
use raytraceresult::Ray2TraceResult;
use vec2::Vec2;

use std::mem::transmute;
use std::slice;

pub struct CherenkovSim {
	pub obstacles: Vec <PolyCapsule>,
	pub player: Ray2,
	pub radius: Fx32,
}

#[repr(C)]
pub struct PodVec2 {
	pub x: i32,
	pub y: i32,
}

#[no_mangle]
pub extern fn cher_new (radius: f32) -> *mut CherenkovSim {
	let ctx = CherenkovSim {
		obstacles: vec! [],
		player: Ray2::new (
			Vec2 {
				x: Fx32::from_q (0, 1),
				y: Fx32::from_q (0, 1),
			},
			Vec2 {
				x: Fx32::from_q (0, 1),
				y: Fx32::from_q (0, 1),
			},
		),
		radius: Fx32::from_float (radius),
	};
	
	unsafe {
		transmute (Box::new (ctx))
	}
}

#[no_mangle]
pub extern fn cher_add_polycapsule (opaque: *mut CherenkovSim, n: i32, points: *const PodVec2) 
{
	let context = unsafe { &mut*opaque };
	let points = unsafe { slice::from_raw_parts (points, n as usize) };
	
	let points: Vec <Vec2 <Fx32>> = points.iter ().map (|pod| {
		Vec2 { x: Fx32 { x: pod.x }, y: Fx32 { x: pod.y } }
	}).collect ();
	
	let capsule = PolyCapsule::new (&points, context.radius);
	
	context.obstacles.push (capsule);
}

#[no_mangle]
pub extern fn cher_step (opaque: *mut CherenkovSim) {
	let context = unsafe { &mut*opaque };
	
	//context.player.start.x = context.player.start.x + Fx32::from_q (1, 1);
	
	step_sim (context);
}

#[no_mangle]
pub extern fn cher_get_player (opaque: *const CherenkovSim) -> PodVec2 {
	let context = unsafe { &*opaque };
	
	let pos = context.player.start;
	
	PodVec2 {
		x: pos.x.x,
		y: pos.y.x,
	}
}

#[no_mangle]
pub extern fn cher_delete (opaque: *mut CherenkovSim) {
	let _context: Box <CherenkovSim> = unsafe {
		transmute (opaque)
	};
}

fn step_sim (ctx: &mut CherenkovSim) {
	let scale = 1;
	
	let radius = Fx32::from_q (20, scale);
	
	let inv_dt = 8;
	let gravity = Vec2::<Fx32> {
		x: Fx32::from_q (0, 1),
		y: Fx32::from_q (1, 1),
	};
	
	let mut clock = Fx32::from_int (0);
	
	let dt = Fx32::from_q (1, inv_dt).to_small ();
	
	let mut remaining_dt = Fx32::from_int (1);
	
	let mut particle = Ray2::new (ctx.player.start, ctx.player.get_dir () + gravity);
	
	for subtick in 0..4 {
		let trace_result = ctx.obstacles.iter ().map (|capsule| {
			let dt_particle = particle.apply_dt (remaining_dt.to_small ());
			
			let point_results = capsule.arcs.iter ().map (|obstacle| ray_trace_arc (&dt_particle, obstacle));
			
			let line_results = capsule.lines.iter ().map (|line| ray_trace_line_2 (&dt_particle, line)); 
			
			point_results.chain (line_results).fold ( Ray2TraceResult::Miss, Ray2TraceResult::fold)
		}).fold (Ray2TraceResult::Miss, Ray2TraceResult::fold);
		
		match trace_result {
			Ray2TraceResult::Miss => {
				particle.start = particle.start + (particle.get_dir () * remaining_dt);
				// Consume the entire remaining tick timestep
				clock = clock + Fx32::from (remaining_dt);
				remaining_dt = Fx32::from_int (0);
			},
			Ray2TraceResult::Pop (ccd_pos, normal) => {
				let reflected_dir = particle.get_dir ().reflect_res (normal, Fx32::from_q (0, 1024).to_small ());
				
				let new_dir = reflected_dir;
				
				particle.start = ccd_pos;
				if particle.get_dir () * normal < 0 {
					particle = Ray2::new (particle.start, new_dir);
				}
				
				// Consume no time - This may lead to time dilation
				// for some objects if we run short of CPU
			},
			Ray2TraceResult::Hit (t, ccd_pos, normal) => {
				particle.start = ccd_pos;
				if particle.get_dir () * normal < 0 {
					particle = Ray2::new (particle.start, particle.get_dir ().reflect_res (normal, Fx32::from_q (512, 1024).to_small ()));
				}
				
				// TODO: only works if dt == 1
				// Consume just the right portion of time
				let consumed_time = remaining_dt * Fx32::from (t);
				remaining_dt = remaining_dt - consumed_time;
				clock = clock + consumed_time;
			},
		};
		
		if remaining_dt <= Fx32::from_int (0) {
			break;
		}
	}
	
	ctx.player = particle;
}

