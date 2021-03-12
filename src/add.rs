use std::env;
use std::fs;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;

fn get_file_mk() -> fs::File {
	let path = match env::current_dir() {
		Ok(_path) => _path,
		Err(error) => panic!("Error extracting working directory {:?}", error),
	};
	let file = fs::OpenOptions::new().read(true)
									 .write(true)
									 .open(path.as_path().join("files.mk"))
										.expect("Error openning file");
	file
}
									
pub fn link_file(fname: &mut String, pattern: &String) {
	let file = get_file_mk();
	let mut buffer = BufReader::new(file);
	let mut content = String::new();
	buffer.read_to_string(&mut content).expect("Error while reading file");
	let mut error = pattern.clone();
	error.push_str(" : field not found");
	let mut idx = content.rfind(pattern).expect(&*error);
	idx = idx + pattern.len();
	fname.push_str(" \\\n");
	content.insert_str(idx, fname);
	let mut file = get_file_mk();
	file.write_all(content.as_bytes()).expect("Error writing in file");
}
