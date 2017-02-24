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
		
		assert! ((a + b).x == 9000, "Fx32 add failed");
		assert! ((a - b).x == 1000, "Fx32 sub failed");
		assert! (-a.x == -5000, "Fx32 neg failed");
		
		let half = Fx32::from_float (0.5f32);
		let quarter = Fx32::from_float (0.25f32);
		let three_quarters = Fx32::from_float (0.75f32);
		
		assert! (half + quarter == three_quarters, "from_float / add failed");
		
		assert! (half * half == quarter, "mul");
		assert! (Fx32::from_float (1.0f32 / 256.0f32) * Fx32::from_float (512.0f32) == Fx32::from_float (2.0f32), "mul");
		
		assert! (Fx32::from_float (255.0f32) * Fx32::from_float (256.0f32) == Fx32::from_float (255.0f32 * 256.0f32), "big * big mul");
		
		assert! (Fx32::from_float (1.0f32 / 256.0f32) * Fx32::from_float (1.0f32 / 128.0f32) == Fx32::from_float (1.0f32 / 128.0f32 / 256.0f32), "small * small mul");
		
		assert! (Fx32::mul_precise (Fx32::from_float (1.0f32 / 1024.0f32), Fx32::from_float (1024.0f32)) == Fx32::from_float (1.0f32), "small * big mul");
    }
}
