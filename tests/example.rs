extern crate lz4;

use std::io::{Write, Read, Error, ErrorKind};
use std::fs::File;
use lz4::encoder::Encoder;

fn read_usize(stream: &mut Read) -> Result<usize, Error> {
	let mut result: usize = 0;
	let mut buffer = [0 as u8; 8];
	match try! (stream.read(&mut buffer)) {
		0 => Ok(0),
		v if v == buffer.len() => {
			for i in 0..buffer.len() {
		 		result += (buffer[i] as usize) << (i * 8);
			}
			Ok(result)
		}
		_ => Err(Error::new(ErrorKind::Other, "Unexpected end of stream")),
	}
}

#[test]
fn test_compression() {
	let mut encoder = Encoder::new(Vec::new(), 1).unwrap();
	let mut buffer: [u8; 64 * 1024] = [0; 64 * 1024];
	let mut source = File::open("tests/a0a2bad4090baa.blk").unwrap();
	loop {
		let len = read_usize(&mut source).unwrap();
		if len == 0 {
			break;
		}
		if source.read(&mut buffer[0..len]).unwrap() != len {
			panic! ("Unexpected end of stream");
		}
		encoder.write(&buffer[0..len]).unwrap();
	}
	let (_, result) = encoder.finish();
	result.unwrap();
}
