mod fx32;

#[cfg(test)]
mod tests {
	use super::fx32::Fx32;
	
    #[test]
    fn it_works() {
		let a = Fx32::new (5000);
		let b = Fx32::new (4000);
		
		assert! (Fx32::add (&a, &b).x == 9000, "It's not exactly NINE THOUSAND!");
    }
}
