extern crate svg;

pub mod circle;
pub mod fx32;
pub mod ray2;
pub mod raytrace;
pub mod vec2;

#[cfg(test)]
mod tests {
	use super::circle::Circle;
	use super::fx32::Fx32;
	use super::ray2::Ray2;
	use super::raytrace;
	use super::vec2::Vec2;
	use super::svg::Document;
	use super::svg::node::element::Path;
	use super::svg::node::element::path::Data;
	
	#[test]
	fn svg () {
		let mut document = Document::new()
			.set("viewBox", (0, 0, 512, 512));
		
		let scale = 1;
		let scale_fx = Fx32::from_int (scale);
		
		let obstacle = vec! [
		Circle {
			center: Vec2 {x: Fx32::from_q (256, scale), y: Fx32::from_q (500, scale)},
			radius: Fx32::from_q (50, scale),
		},
		Circle {
			center: Vec2 {x: Fx32::from_q (81, scale), y: Fx32::from_q (512 - 232, scale)},
			radius: Fx32::from_q (40, scale),
		},
		Circle {
			center: Vec2 {x: Fx32::from_q (404, scale), y: Fx32::from_q (512 - 41, scale)},
			radius: Fx32::from_q (60, scale),
		},
		];
		
		for x in 512 - 110..512 + 110 {
			let mut particle = Ray2 {
				start: Vec2 {
					x: Fx32::from_q (x, scale * 2),
					y: Fx32::from_q (0, scale)
				},
				dir: Vec2 {
					x: Fx32::from_q (0, scale),
					y: Fx32::from_q (100, scale),
				},
			};
			
			let mut data = Data::new().move_to(((particle.start.x * scale_fx).to_i32 (), (particle.start.y * scale_fx).to_i32 ()));
			
			for step in 0..10 {
				let trace_result = obstacle.iter ().map(|obstacle: &Circle| raytrace::ray_trace_circle_2 (&particle, obstacle)).fold ( raytrace::Ray2TraceResult::Miss, raytrace::fold_closer_result);
				
				match trace_result {
					raytrace::Ray2TraceResult::Miss => {
						particle.start = particle.start + particle.dir;
					},
					raytrace::Ray2TraceResult::Hit (t, ccd_pos, normal) => {
						particle.start = ccd_pos;
						particle.dir = particle.dir.reflect (normal);
						//particle.dir = normal;
					},
				};
				
				data = data.line_to(((particle.start.x * scale_fx).to_i32 (), (particle.start.y * scale_fx).to_i32 ()));
			}
			
			//data = data.close ();
			
			let path = Path::new()
				.set("fill", "none")
				.set("stroke", "black")
				.set("stroke-width", 0.5)
				.set("d", data);
			
			document = document.add(path);
		}
		
		super::svg::save("image.svg", &document).unwrap();
	}
	
	#[test]
	fn vec () {
		let a = Vec2 {x: Fx32::from_int (2), y: Fx32::from_int (3)};
		let b = Vec2 {x: Fx32::from_int (4), y: Fx32::from_int (5)};
		let c = Vec2 {x: Fx32::from_int (6), y: Fx32::from_int (8)};
		
		assert_eq! (a + b, c, "Vector add");
		
		assert_eq! (
			Vec2 {x: Fx32::from_int (3), y: Fx32::from_int (4)}.length (), 
			Fx32::from_int (5), 
			"pythagorean triple");
		
		assert_eq! (
			Vec2 {x: Fx32::from_int (10), y: Fx32::from_int (1)}.cross (),
			Vec2 {x: Fx32::from_int (-1), y: Fx32::from_int (10)},
			"2D cross product");
		
		assert_eq! (
			Vec2 {x: Fx32::from_int (5), y: Fx32::from_int (5)}.reflect (Vec2 {x: Fx32::from_int (0), y: Fx32::from_int (-1)}),
			Vec2 {x: Fx32::from_int (5), y: Fx32::from_int (-5)},
			"reflect");
	}
	
	#[test]
	fn it_works() {
		let a = Fx32::new (5000);
		let b = Fx32::new (4000);
		
		assert! (a == a, "eq");
		assert! (b == b, "eq");
		assert! (a != b, "eq");
		
		assert_eq! ((a + b).x, 9000, "Fx32 add failed");
		assert_eq! ((a - b).x, 1000, "Fx32 sub failed");
		assert_eq! (-a.x, -5000, "Fx32 neg failed");
		
		//assert_eq! (65536, super::fx32::DENOMINATOR, "Denominator is wrong");
		//assert_eq! (65536, Fx32::from_float (1.0f32).x, "Denominator applied wrong");
		
		let half = Fx32::from_float (0.5f32);
		let quarter = Fx32::from_float (0.25f32);
		let three_quarters = Fx32::from_float (0.75f32);
		
		assert_eq! (half + quarter, three_quarters, "from_float / add failed");
		
		assert! (half * half == quarter, "mul");
		assert! (Fx32::from_float (1.0f32 / 64.0f32) * Fx32::from_float (128.0f32) == Fx32::from_float (2.0f32), "mul");
		
		let biggest_root = 181.0f32;
		assert_eq! (
			Fx32::from_float (biggest_root) * Fx32::from_float (biggest_root), 
			Fx32::from_float (biggest_root * biggest_root), 
			"big * big mul");
		
		assert_eq! (
			Fx32::from_float (1.0f32 / 64.0f32) * Fx32::from_float (1.0f32 / 32.0f32),
			Fx32::from_float (1.0f32 / 32.0f32 / 64.0f32), 
			"small * small mul");
		
		assert_eq! (
			Fx32::mul_precise (Fx32::from_float (1.0f32 / 1024.0f32), Fx32::from_float (1024.0f32)),
			Fx32::from_float (1.0f32), 
			"small * big mul");
		
		/*
		Remember, i32 is signed, so our 16.16 is really 15.16
		plus one sign bit
		If we scale this to meters, we have no more than
		32 km negative and positive, with 1/65 millimeter granularity.
		For my plan, this is fine, if I stay within 10 km each way the
		math should be fine and that's a bigger world than I could create.
		
		Update: Our 20.12 is actually 1.19.12
		We have no more than 524,288 meters negative and positive, with
		1/4 millimeter granularity.
		
		I am trying 20.12 not because I need a bigger world but because
		I want more headroom for multiplying large numbers.
		*/
		assert_eq! (
			Fx32::mul_64 (Fx32::from_float (1.0f32 / 2048.0f32), Fx32::from_float (16384.0f32)),
			Fx32::from_float (8.0f32), 
			"mul_64");
		
		assert_eq! (
			Fx32::from_float (1.0f32 / 1024.0f32).mul_small (
			Fx32::from_float (0.5f32)),
			Fx32::from_float (1.0f32 / 2048.0f32), 
			"mul_small");
		
		assert_eq! (
			Fx32::from_float (1.0f32).mul_small (
			Fx32::from_float (1.5f32)),
			Fx32::from_float (1.5f32), 
			"mul_small");
		
		assert_eq! (
			Fx32::from_int (512) / Fx32::from_int (8),
			Fx32::from_int (64),
			"div_64");
		
		assert_eq! (
			Fx32::from_int (1).square_64 (),
			Fx32::from_int (1), 
			"square");
		assert_eq! (
			Fx32::from_int (1).sqrt_64 (),
			Fx32::from_int (1), 
			"square_root");
		
		assert_eq! (
			Fx32::from_int (9).square_64 (),
			Fx32::from_int (81), 
			"square");
		assert_eq! (
			Fx32::from_int (9).sqrt_64 (),
			Fx32::from_int (3), 
			"sqrt");
		
		// Biggest POT I can square with 20.12 is 512
		// With 16.16 is 128
		assert_eq! (
			Fx32::from_int (128).square_64 (),
			Fx32::from_int (128 * 128), 
			"big square");
		
		assert_eq! (
			Fx32::from_int (128 * 128).sqrt_64 (),
			Fx32::from_int (128), 
			"big sqrt");
		
		assert_eq! (
			Fx32::from_int (511) * Fx32::from_q (1, 10).to_small (),
			Fx32::from_q (3348583, 65536),
			"Fx32 * Fx32Small");
		
		assert_eq! (
			Fx32::from_int (16383) * Fx32::from_q (1, 10).to_small (),
			Fx32::from_q (107357799, 65536),
			"Fx32 * Fx32Small");
		
		assert_eq! (
			Fx32::from_int (-1).abs (),
			Fx32::from_int (1),
			"abs");
		
		
    }
}
