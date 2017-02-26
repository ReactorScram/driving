pub mod fx32;

#[cfg(test)]
mod tests {
	use super::fx32::Fx32;
	
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
		
		assert_eq! (65536, super::fx32::DENOMINATOR, "Denominator is wrong");
		assert_eq! (65536, Fx32::from_float (1.0f32).x, "Denominator applied wrong");
		
		let half = Fx32::from_float (0.5f32);
		let quarter = Fx32::from_float (0.25f32);
		let three_quarters = Fx32::from_float (0.75f32);
		
		assert! (half + quarter == three_quarters, "from_float / add failed");
		
		assert! (half * half == quarter, "mul");
		assert! (Fx32::from_float (1.0f32 / 256.0f32) * Fx32::from_float (512.0f32) == Fx32::from_float (2.0f32), "mul");
		
		assert! (Fx32::from_float (181.0f32) * Fx32::from_float (181.0f32) == Fx32::from_float (181.0f32 * 181.0f32), "big * big mul");
		
		assert! (Fx32::from_float (1.0f32 / 256.0f32) * Fx32::from_float (1.0f32 / 128.0f32) == Fx32::from_float (1.0f32 / 128.0f32 / 256.0f32), "small * small mul");
		
		assert! (Fx32::mul_precise (Fx32::from_float (1.0f32 / 1024.0f32), Fx32::from_float (1024.0f32)) == Fx32::from_float (1.0f32), "small * big mul");
		
		/*
		Remember, i32 is signed, so our 16.16 is really 15.16
		plus one sign bit
		If we scale this to meters, we have no more than
		32 km negative and positive, with 1/65 millimeter granularity.
		For my plan, this is fine, if I stay within 10 km each way the
		math should be fine and that's a bigger world than I could create.
		*/
		assert! (Fx32::mul_64 (Fx32::from_float (1.0f32 / 16384.0f32), Fx32::from_float (16384.0f32)) == Fx32::from_float (1.0f32), "mul_64");
		
		assert! (Fx32::mul_small (Fx32::from_float (1.0f32 / 8192.0f32), Fx32::from_float (0.5f32)) == Fx32::from_float (1.0f32 / 16384.0f32), "mul_small");
		
		assert! (Fx32::mul_small (Fx32::from_float (1.0f32), Fx32::from_float (1.5f32)) == Fx32::from_float (1.5f32), "mul_small");
    }
}
