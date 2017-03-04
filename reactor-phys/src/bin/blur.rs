// Farbfeld filter that applies a cool blurring effect over images
// Usage: png2ff < greyscale.png | blur | ff2png > false-colored.png

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

extern crate rayon;
use rayon::prelude::*;

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

fn gaussian_filter (std_dev: f64) -> Vec <f64> {
	let scale = (2.0 * 3.14159265358979f64).sqrt () * std_dev;
	let radius = 3.0 * std_dev;
	let pow_scale = 2.0 * std_dev * std_dev;
	
	(-radius.ceil () as i32..(radius.ceil () + 1.0) as i32).map (|x| {
		let x = x as f64;
		((-x * x) / pow_scale).exp () / scale
	}).collect ()
}

struct ScanlineJob <'a> {
	pub y: i32,
	pub row_chunk: &'a mut [f64],
}

fn get_filter_offset (filter_len: i32) -> i32 {
	-(filter_len - 1) / 2
}

fn blur_hor (input_plane: &HdrPlane, filter: &[f64]) -> HdrPlane {
	let offset = get_filter_offset (filter.len () as i32);
	
	let sz = input_plane.size;
	
	let mut pixels = vec! [0.0f64; (sz.x * sz.y) as usize];
	
	(0..sz.y).zip (pixels.chunks_mut (sz.x as usize)).map (|(y, row_chunk)| ScanlineJob { y: y, row_chunk: row_chunk }).collect::<Vec <ScanlineJob>> ().par_iter_mut ().for_each (|job| {
		let scanline_index = job.y * sz.x;
		
		for x in 0..sz.x {
			let offset_2 = x + offset;
			
			job.row_chunk [x as usize] = (sz.clamp_x (x + 0 + offset) - offset_2..sz.clamp_x (x + filter.len () as i32 + offset) - offset_2).map (|i| filter [i as usize] * input_plane.pixels [(scanline_index + i + offset_2) as usize]).sum ();
		}
	});
	
	HdrPlane {
		size: sz,
		pixels: pixels,
	}
}

fn blur_vert (input_plane: &HdrPlane, filter: &[f64]) -> HdrPlane {
	let offset = get_filter_offset (filter.len () as i32);
	
	let sz = input_plane.size;
	
	let mut pixels = vec! [0.0f64; (sz.x * sz.y) as usize];
	
	(0..sz.y).zip (pixels.chunks_mut (sz.x as usize)).map (|(y, row_chunk)| ScanlineJob { y: y, row_chunk: row_chunk }).collect::<Vec <ScanlineJob>> ().par_iter_mut ().for_each (|job| {
		for x in 0..sz.x {
			let offset_2 = job.y + offset;
			
			job.row_chunk [x as usize] = (sz.clamp_y (job.y + 0 + offset) - offset_2..sz.clamp_y (job.y + filter.len () as i32 + offset) - offset_2).map (|i| filter [i as usize] * input_plane.pixels [( sz.index (&PlaneCoord {x: x, y: i + offset_2})) as usize]).sum ();
		}
	});
	
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

fn write_farbfeld <T, I> (writer: &mut T, size: &PlaneCoord, pixels: I ) where T: WriteBytesExt, I: Iterator <Item = FfPixel>
{
	writer.write ("farbfeld".as_bytes ()).expect ("printing farbfeld");
	
	writer.write_u32::<BigEndian> (size.x as u32).unwrap ();
	writer.write_u32::<BigEndian> (size.y as u32).unwrap ();
	
	for pixel in pixels.take ((size.x * size.y) as usize) {
		write_pixel (writer, &pixel);
	}
}

fn blur (input_plane: &HdrPlane, filter: &[f64]) -> HdrPlane {
	blur_vert (&blur_hor (input_plane, filter), filter)
	//blur_hor (input_plane, filter)
	//blur_vert (input_plane, filter)
}

fn double_to_ff (val: f64) -> u16 {
	let max = 1.0;
	(val.min (max) * 65535.0 / max) as u16
}

fn to_linear (rgb: &[f64;3]) -> [f64;3] {
	[rgb [0] * rgb [0], rgb [1] * rgb [1], rgb [2] * rgb [2]]
}

fn to_gamma (rgb: &[f64;3]) -> [f64;3] {
	[rgb [0].sqrt (), rgb [1].sqrt (), rgb [2].sqrt ()]
}

fn alpha_blend (rgb_src: &[f64;3], rgb_dest: &[f64;3], alpha_src: f64) -> [f64;3] 
{
	let mut rgb = [0.0, 0.0, 0.0];
	
	let alpha_dest = 1.0 - alpha_src;
	
	for i in 0..3 {
		let x = rgb_src [i] * alpha_src + rgb_dest [i] * alpha_dest;
		rgb [i] = x;
	}
	
	rgb
}

fn main () {
	// Reader
	let mut reader = BufReader::new (io::stdin ());
	
	let input_plane = read_farbfeld_grey (&mut reader);
	
	let blurry_1 = blur (&input_plane, &gaussian_filter (0.5));
	let blurry_20 = blur (&input_plane, &gaussian_filter (5.0));
	let blurry_40 = blur (&input_plane, &gaussian_filter (40.0));
	
	let sum_iter = (0..(input_plane.size.x * input_plane.size.y) as usize).map (|i| {
		let rgb = to_linear (&[7.0 / 256.0, 1.0 / 256.0, 21.0 / 256.0]);
		
		let rgb = alpha_blend (&to_linear (&[210.0 / 200.0, 0.0 / 200.0, 156.0 / 200.0]), &rgb, blurry_40.pixels [i]);
		
		let rgb = alpha_blend (&to_linear (&[71.0 / 200.0, 203.0 / 200.0, 256.0 / 200.0]), &rgb, blurry_20.pixels [i]);
		
		let rgb = alpha_blend (&[1.0, 1.0, 1.0], &rgb, blurry_1.pixels [i]);
		
		let rgb = to_gamma (&rgb);
		
		FfPixel {
			r: double_to_ff (rgb [0]),
			g: double_to_ff (rgb [1]),
			b: double_to_ff (rgb [2]),
			a: 65535,
		}
	});
	
	// Write
	{
		let mut writer = BufWriter::new (io::stdout ());
		
		write_farbfeld (&mut writer, &input_plane.size, sum_iter);
	}
	
	let test_filter = gaussian_filter (1.0);
	
	writeln!(&mut io::stderr(), "filter {:?}, {:?}", test_filter, get_filter_offset (test_filter.len () as i32)).expect("failed printing to stderr");
}
