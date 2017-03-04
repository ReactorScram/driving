// Farbfeld filter that applies a cool blurring effect over images
// Usage: png2ff < greyscale.png | blur | ff2png > false-colored.png

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

#[derive (Clone, Copy)]
struct FfPixel {
	pub r: u16,
	pub g: u16,
	pub b: u16,
	pub a: u16,
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

fn main () {
	let mut reader = BufReader::new (io::stdin ());
	
	let mut magic = [0; 8];
	reader.read (&mut magic [..]);
	
	// TODO: Assert that magic == 'farbfeld'
	//assert_eq! (magic, &"farbfeld");
	
	let width = reader.read_u32::<BigEndian>().unwrap();
	let height = reader.read_u32::<BigEndian>().unwrap();
	
	writeln!(&mut io::stderr(), "{} x {}", width, height).expect("failed printing to stderr");
	
	let mut pixels = Vec::new ();
	
	for i in 0..width * height {
		pixels.push (read_pixel (&mut reader).g);
	}
	
	{
		let mut writer = BufWriter::new (io::stdout ());
		
		writer.write ("farbfeld".as_bytes ()).expect ("printing farbfeld");
		
		writer.write_u32::<BigEndian> (width).unwrap ();
		writer.write_u32::<BigEndian> (height).unwrap ();
		
		for i in 0..(width * height) as usize {
			let val = pixels [i];
			
			write_pixel (&mut writer, &FfPixel {
				r: val,
				g: 0,
				b: val,
				a: 65535,
			});
		}
	}
}
