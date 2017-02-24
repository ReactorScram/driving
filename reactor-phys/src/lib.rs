pub mod fx32;

#[cfg(test)]
mod tests {
	use super::fx32::Fx32;
	
    #[test]
    fn it_works() {
		let a = Fx32::new (5000);
		let b = Fx32::new (4000);
		
		assert! ((a + b).x == 9000, "Fx32 add failed");
		assert! ((a + b).x == 9000, "Fx32 add failed");
    }
}
