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

#[derive (Clone, Copy)]
struct PlaneCoord {
	pub x: i32,
	pub y: i32,
}

impl PlaneCoord {
	pub fn contains (&self, x: i32, y: i32) -> bool {
		x >= 0 && x < self.x &&
		y >= 0 && y < self.y
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

fn pascal (n: u32) -> Vec <u32> {
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

fn blur_hor (input_plane: &HdrPlane, filter: &Vec <u32>) -> HdrPlane {
	let offset = -(filter.len () as i32 - 1) / 2;
	let filter_sum: u32 = filter.iter ().sum ();
	// Rust plz
	let filter_sum = filter_sum as f64;
	let filter_f: Vec <f64> = filter.iter ().map (|x| *x as f64 / filter_sum).collect ();
	
	let mut pixels = vec![];
	for y in 0..input_plane.size.y {
		for x in 0..input_plane.size.x {
			let dest_index = input_plane.size.clamped_index (&PlaneCoord { 
				x: x, 
				y: y 
			});
			
			pixels.push (0.0f64);
			
			for i in 0..filter_f.len () {
				let src_index = input_plane.size.clamped_index (&PlaneCoord { 
					x: x + i as i32 + offset, 
					y: y 
				});
				
				pixels [dest_index as usize] += filter_f [i] * input_plane.pixels [src_index as usize];
			}
		}
	}
	
	HdrPlane {
		size: input_plane.size,
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
	
	let mut pixels = Vec::new ();
	
	for _ in 0 .. width * height {
		let gamma = read_pixel (reader).g as f64 / 65536.0;
		pixels.push (gamma * gamma);
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
	writer.write ("farbfeld".as_bytes ()).expect ("printing farbfeld");
	
	writer.write_u32::<BigEndian> (plane.size.x as u32).unwrap ();
	writer.write_u32::<BigEndian> (plane.size.y as u32).unwrap ();
	
	for i in 0..plane.size.x * plane.size.y {
		let val = (plane.pixels [i as usize].sqrt () * 65535.0) as u16;
		
		write_pixel (writer, &FfPixel {
			r: val,
			g: val,
			b: val,
			a: 65535,
		});
	}
}

fn main () {
	// Reader
	let mut reader = BufReader::new (io::stdin ());
	
	let input_plane = read_farbfeld_grey (&mut reader);
	
	let filter = pascal (31);
	
	let input_plane = blur_hor (&input_plane, &filter);
	let input_plane = blur_hor (&input_plane, &filter);
	let input_plane = blur_hor (&input_plane, &filter);
	let input_plane = blur_hor (&input_plane, &filter);
	let input_plane = blur_hor (&input_plane, &filter);
	let input_plane = blur_hor (&input_plane, &filter);
	
	// Write
	{
		let mut writer = BufWriter::new (io::stdout ());
		
		write_farbfeld_plane (&mut writer, &input_plane);
	}
	
	writeln!(&mut io::stderr(), "filter {:?}", filter).expect("failed printing to stderr");
}
