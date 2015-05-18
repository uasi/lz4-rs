extern crate lz4;

use std::io::{Write, Read};
use std::fs::File;
use lz4::encoder::Encoder;

fn read_cache(path: &Path, paths: &Vec<PathBuf>) -> Result<OutputInfo, Error> {
	let mut file = try! (OpenOptions::new().read(true).write(true).open(Path::new(path)));
	try! (file.write(&[4]));
	try! (file.seek(SeekFrom::Start(0)));
	let mut stream = try! (lz4::Decoder::new (file));


	if try! (read_exact(&mut stream, HEADER.len())) != HEADER {
		return Err(Error::new(ErrorKind::InvalidInput, CacheError::InvalidHeader(path.to_path_buf())));
	}
	if try! (read_usize(&mut stream)) != paths.len() {
		return Err(Error::new(ErrorKind::InvalidInput, CacheError::PackedFilesMismatch(path.to_path_buf())));
	} 
	for path in paths.iter() {
		let mut file = try! (File::create(path));
		loop {
			let size = try! (read_usize(&mut stream));
			if size == 0 {break;}
			let block = try! (read_exact(&mut stream, size));
			try! (file.write_all(&block));
		}
	}
	let output = try! (read_output(&mut stream));
	if try! (read_exact(&mut stream, FOOTER.len())) != FOOTER {
		return Err(Error::new(ErrorKind::InvalidInput, CacheError::InvalidFooter(path.to_path_buf())));
	}
	Ok(output)
}

#[test]
fn test_compression() {
	let mut encoder = Encoder::new(Vec::new(), 1).unwrap();
	let mut buffer: [u8; 64 * 1024] = [0; 64 * 1024];
	let mut source = File::open("tests/0595f71fd47dfc.raw").unwrap();
	encoder.write(&HEADER).unwrap();
	encoder.write(&[0x00, 0x00, 0x00, 0x01]).unwrap();
	loop {
		encoder.write(&[0x00, 0x01, 0x00, 0x00]).unwrap();
		match source.read(&mut buffer).unwrap() {
			0 => {break;}
			s => {encoder.write(&buffer[0..s]).unwrap();}
		}
	}
	let (_, result) = encoder.finish();
	result.unwrap();
}
