pub mod fx32;
pub mod vec2;
pub mod vec4;

#[cfg(test)]
mod tests {
	use super::fx32::Fx32;
	use super::vec2::Vec2;
	
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
		
		let biggest_root = 724.0f32;
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
			Fx32::mul_small (Fx32::from_float (1.0f32 / 1024.0f32), 
			Fx32::from_float (0.5f32)),
			Fx32::from_float (1.0f32 / 2048.0f32), 
			"mul_small");
		
		assert_eq! (
			Fx32::mul_small (Fx32::from_float (1.0f32), 
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
		
		// Biggest POT I can square with 20.12, but not the biggest int
		assert_eq! (
			Fx32::from_int (512).square_64 (),
			Fx32::from_int (512 * 512), 
			"big square");
		
		assert_eq! (
			Fx32::from_int (512 * 512).sqrt_64 (),
			Fx32::from_int (512), 
			"big sqrt");
    }
}
