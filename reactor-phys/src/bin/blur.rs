// Farbfeld filter that applies a cool blurring effect over images
// Usage: png2ff < greyscale.png | blur | ff2png > false-colored.png

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

#[derive (Clone, Copy)]
struct FfPixel {
	pub r: u16,
	pub g: u16,
	pub b: u16,
	pub a: u16,
}

#[derive (Clone, Copy, Debug, PartialEq)]
struct PlaneCoord {
	pub x: i32,
	pub y: i32,
}

impl PlaneCoord {
	pub fn contains_x (&self, x: i32) -> bool {
		x >= 0 && x < self.x
	}
	
	pub fn contains_y (&self, y: i32) -> bool {
		y >= 0 && y < self.y
	}
	
	pub fn contains (&self, x: i32, y: i32) -> bool {
		self.contains_x (x) &&
		self.contains_y (y)
	}
	
	pub fn clamp_x (&self, x: i32) -> i32 {
		return if x < 0 {
			0
		}
		else if x >= self.x {
			self.x - 1
		}
		else {
			x
		}
	}
	
	pub fn clamp_y (&self, y: i32) -> i32 {
		return if y < 0 {
			0
		}
		else if y >= self.y {
			self.y - 1
		}
		else {
			y
		}
	}
	
	pub fn clamp (&self, pc: &PlaneCoord) -> PlaneCoord {
		PlaneCoord {
			x: self.clamp_x (pc.x),
			y: self.clamp_y (pc.y),
		}
	}
	
	pub fn index (&self, pc: &PlaneCoord) -> i32 {
		let pc = self.clamp (pc);
		
		pc.y * self.x + pc.x
	}
	
	pub fn clamped_index (&self, pc: &PlaneCoord) -> i32 {
		self.index (&self.clamp (pc))
	}
}

struct HdrPlane {
	pub size: PlaneCoord,
	pub pixels: Vec <f64>,
}

impl HdrPlane {
	pub fn new (size: PlaneCoord) -> HdrPlane {
		HdrPlane {
			size: size,
			pixels: vec! [0.0; (size.x * size.y) as usize],
		}
	}
}

fn read_pixel <T> (reader: &mut T) -> FfPixel where T: ReadBytesExt {
	FfPixel {
		r: reader.read_u16::<BigEndian> ().unwrap (),
		g: reader.read_u16::<BigEndian> ().unwrap (),
		b: reader.read_u16::<BigEndian> ().unwrap (),
		a: reader.read_u16::<BigEndian> ().unwrap (),
	}
}

fn write_pixel <T> (writer: &mut T, pix: &FfPixel) where T: WriteBytesExt {
	writer.write_u16::<BigEndian> (pix.r).unwrap ();
	writer.write_u16::<BigEndian> (pix.g).unwrap ();
	writer.write_u16::<BigEndian> (pix.b).unwrap ();
	writer.write_u16::<BigEndian> (pix.a).unwrap ();
}

fn pascal (n: u32) -> Vec <u64> {
	match n {
		0 => {
			vec! []
		}
		1 => {
			vec! [1]
		},
		2 => {
			vec! [1, 1]
		},
		3 => {
			vec! [1, 2, 1]
		},
		4 => {
			vec! [1, 3, 3, 1]
		}
		5 => {
			vec! [1, 4, 6, 4, 1]
		},
		_ => {
			let step = 5;
			let prev_row = pascal (n - step + 1);
			let step_row = pascal (step);
			
			let mut output = vec! [];
			for _ in 0..n {
				output.push (0);
			}
			
			for a in 0..prev_row.len () {
				for b in 0..step_row.len () {
					output [a + b] += prev_row [a] * step_row [b];
				}
			}
			
			output
		}
	}
}

fn blur_hor (input_plane: &HdrPlane, filter: &Vec <u64>) -> HdrPlane {
	let offset = -(filter.len () as i32 - 1) / 2;
	let filter_sum: u64 = filter.iter ().sum ();
	// Rust plz
	let filter_sum = filter_sum as f64;
	let filter_f: Vec <f64> = filter.iter ().map (|x| *x as f64 / filter_sum).collect ();
	
	let sz = input_plane.size;
	
	let mut pixels = vec![];
	for y in 0..sz.y {
		for x in 0..sz.x {
			let dest_index = sz.clamped_index (&PlaneCoord { 
				x: x, 
				y: y 
			});
			
			pixels.push (0.0f64);
			
			for src_x in sz.clamp_x (x + 0 + offset)..sz.clamp_x (x + filter_f.len () as i32 - 1 + offset) 
			{
				let src_index = sz.clamped_index (&PlaneCoord { 
					x: src_x, 
					y: y 
				});
				
				pixels [dest_index as usize] += filter_f [(src_x - x - offset) as usize] * input_plane.pixels [src_index as usize];
			}
		}
	}
	
	HdrPlane {
		size: sz,
		pixels: pixels,
	}
}

fn blur_vert (input_plane: &HdrPlane, filter: &Vec <u64>) -> HdrPlane {
	let offset = -(filter.len () as i32 - 1) / 2;
	let filter_sum: u64 = filter.iter ().sum ();
	// Rust plz
	let filter_sum = filter_sum as f64;
	let filter_f: Vec <f64> = filter.iter ().map (|x| *x as f64 / filter_sum).collect ();
	
	let sz = input_plane.size;
	
	let mut pixels = vec![];
	for y in 0..sz.y {
		for x in 0..sz.x {
			let dest_index = sz.clamped_index (&PlaneCoord { 
				x: x, 
				y: y 
			});
			
			pixels.push (0.0f64);
			
			for src_y in sz.clamp_y (y + 0 + offset)..sz.clamp_y (y + filter_f.len () as i32 - 1 + offset) 
			{
				let src_index = sz.clamped_index (&PlaneCoord { 
					x: x, 
					y: src_y, 
				});
				
				pixels [dest_index as usize] += filter_f [(src_y - y - offset) as usize] * input_plane.pixels [src_index as usize];
			}
		}
	}
	
	HdrPlane {
		size: sz,
		pixels: pixels,
	}
}

fn read_farbfeld_grey <T> (reader: &mut T) -> HdrPlane where T: ReadBytesExt {
	let mut magic = [0; 8];
	reader.read (&mut magic [..]).unwrap ();
	
	// TODO: Assert that magic == 'farbfeld'
	//assert_eq! (magic, &"farbfeld");
	
	let width = reader.read_u32::<BigEndian>().unwrap() as i32;
	let height = reader.read_u32::<BigEndian>().unwrap() as i32;
	
	writeln!(&mut io::stderr(), "{} x {}", width, height).expect("failed printing to stderr");
	
	let mut pixels = vec![0.0; (width * height) as usize];
	
	for i in 0 .. width * height {
		let gamma = read_pixel (reader).g as f64 / 65536.0;
		pixels [i as usize] = gamma * gamma;
	}
	
	HdrPlane {
		size: PlaneCoord {
			x: width,
			y: height,
		},
		pixels: pixels,
	}
}

fn write_farbfeld_plane <T> (writer: &mut T, plane: &HdrPlane) where T: WriteBytesExt 
{
	write_farbfeld (writer, &plane.size, plane.pixels.iter ().map (|x| {
		let val = (x.sqrt ().min (1.0) * 65535.0) as u16;
		
		FfPixel {
			r: val,
			g: val,
			b: val,
			a: 65535,
		}
	}));
}

fn double_to_ff (val: f64) -> u16 {
	(val.sqrt ().min (1.0) * 65535.0) as u16
}

fn write_farbfeld <T, I> (writer: &mut T, size: &PlaneCoord, pixels: I ) where T: WriteBytesExt, I: Iterator <Item = FfPixel>
{
	writer.write ("farbfeld".as_bytes ()).expect ("printing farbfeld");
	
	writer.write_u32::<BigEndian> (size.x as u32).unwrap ();
	writer.write_u32::<BigEndian> (size.y as u32).unwrap ();
	
	for pixel in pixels.take ((size.x * size.y) as usize) {
		write_pixel (writer, &pixel);
	}
}

fn blur (input_plane: &HdrPlane, filter: &Vec <u64>) -> HdrPlane {
	blur_vert (&blur_hor (input_plane, filter), filter)
}

fn blur_n (input_plane: &HdrPlane, filter: &Vec <u64>, n: u32) -> HdrPlane {
	match n {
		0 => {
			// I know this is dumb
			panic! ("Can't blur an image 0 times")
		},
		1 => {
			blur (input_plane, filter)
		},
		_ => {
			blur (&blur_n (input_plane, filter, n - 1), filter)
		},
	}
}

fn main () {
	// Reader
	let mut reader = BufReader::new (io::stdin ());
	
	let filter = pascal (41);
	
	let input_plane = read_farbfeld_grey (&mut reader);
	let blurry_10 = blur_n (&input_plane, &filter, 10);
	let blurry_20 = blur_n (&blurry_10, &filter, 10);
	
	let sum = [&input_plane, &blurry_20].iter ().fold (
	HdrPlane::new (input_plane.size), 
	|accum, b| {
		assert_eq!(accum.size, b.size);
		HdrPlane {
			size: accum.size,
			pixels: accum.pixels.iter ().zip (b.pixels.iter ()).map (|(a, b)| a + b).collect (),
		}
	});
	
	let sum_iter = (0..(input_plane.size.x * input_plane.size.y) as usize).map (|i| FfPixel {
		r: double_to_ff (input_plane.pixels [i]),
		g: double_to_ff (blurry_10.pixels [i]),
		b: double_to_ff (blurry_20.pixels [i]),
		a: 65535,
	});
	
	// Write
	{
		let mut writer = BufWriter::new (io::stdout ());
		
		write_farbfeld (&mut writer, &input_plane.size, sum_iter);
	}
	
	writeln!(&mut io::stderr(), "filter {:?}", filter).expect("failed printing to stderr");
}
