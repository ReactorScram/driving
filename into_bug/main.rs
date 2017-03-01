struct Inches {
	pub inches: f64,
}

struct Feet {
	pub ft: f64,
}

struct Yards {
	pub yd: f64,
}

impl Into <Feet> for Inches {
	fn into (self) -> Feet {
		Feet { ft: self.inches / 12.0 }
	}
}

impl Into <Yards> for Inches {
	fn into (self) -> Yards {
		Yards { yd: self.inches / 12.0 / 3.0 }
	}
}

fn main () {
	let height = Inches {inches: 68.0};
	
	println! ("Inches: {}", height.inches);
	println! ("Feet: {}", height.into ().ft);
	println! ("Yards: {}", height.into::<Yards> ().yd);
}
