extern crate reactor_phys;
use reactor_phys::raytrace;
use reactor_phys::fx32::Fx32;

fn main () {
	/*
	for i in 0..16 {
		let filename = format! ("lines-{}.obj", i);
		raytrace::test_ray_trace (&filename, Fx32::from_q (i, 4)).unwrap ();
	}
	*/
	
	raytrace::test_ray_trace ("lines.obj", Fx32::from_q (0, 1)).unwrap ();
}
