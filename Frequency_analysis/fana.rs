use std::fs::File;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
	let args: Vec<String> = env::args().collect();

	let mut reader = BufReader::new(File::open(&args[1])?);
	let mut buf = [0; 1];
	let mut _hcnt : [u64; 256] = [0; 256];

	let mut c0cnt : u32 = 0;
	let mut cfcnt : u32 = 0;
	let mut count : u32 = 0;

	let mut cnt00: Vec<u32> = Vec::new();
	let mut cntff: Vec<u32> = Vec::new();

	loop {
		match reader.read(&mut buf)? {
			0 => break,
			n => {
				let buf = &buf[..n];
				let bval = buf[0] as usize;
				_hcnt[bval] += 1;
				count += 1;

				if bval == 0 {
					c0cnt += 1;
				}
				else if bval == 255 {
					cfcnt += 1;
				}

				if count == 65536 {
					count = 0;
					cnt00.push(c0cnt);
					cntff.push(cfcnt);
					c0cnt = 0;
					cfcnt = 0;
				}
			}
		}								
	}

	for i in 0..256 {
		println!("{}, {}", i, _hcnt[i]);
	}

	let mut f = fs::File::create("res.csv")?;
	for i in 0..cnt00.len() {
		f.write((i * 1024).to_string().as_bytes())?;
		f.write(", ".as_bytes())?;
		f.write(cnt00[i].to_string().as_bytes())?;
		f.write(", ".as_bytes())?;
		f.write(cntff[i].to_string().as_bytes())?;
		f.write("\n".as_bytes())?;
	}

	for i in 0..256 {
		f.write(i.to_string().as_bytes())?;
		f.write(", ".as_bytes())?;
		f.write(_hcnt[i].to_string().as_bytes())?;
		f.write("\n".as_bytes())?;
	}

	println!("DONE");

	Ok(())
}
