mod fx32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
		let a = fx32.Fx32 (5000);
		let b = fx32.Fx32 (4000);
		
		assert! (fx32.add (a, b).x == 9000, "It's not exactly NINE THOUSAND!");
    }
}
