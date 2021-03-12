use std::io::ErrorKind;
use std::fs;
use std::io::Write;
use std::env;

#[path = "workspace_format.rs"]
mod workspace_format;

const DIR_LIST: [&str; 7] = ["include",
							"include/template",
							"include/classes",
							"srcs",
							"srcs/classes",
							"lib",
							"objs"];

pub fn create_dir(dir: &str) {
	let path = env::current_dir();
	let path = match path {
		Ok(_path) => _path,
		Err(error) => panic!("Error while exporting directory {:?}", error),
	};
	fs::create_dir(path.as_path().join(dir)).unwrap_or_else(|error| {
		if error.kind() != ErrorKind::AlreadyExists {
			panic!("Couldn't create directory {:?}", error);
		} else {
			println!("{}: already exists", dir);
		}
	});
}

pub fn create_file(location: &str, file: &str, content: &[u8]) {
	let path = match env::current_dir() {
		Ok(_path) => _path,
		Err(error) => panic!("Error while exporting directory {:?}", error),
	};
	let file_path = fs::OpenOptions::new().write(true)
									.create_new(true)
									.open(path.as_path().join(location).join(file));
	if file_path.is_ok() {
		let mut file_path = file_path.unwrap();
		file_path.write_all(content).expect("Couldn't write in file");
	} else {
		let error = file_path.unwrap_err();
		match error.kind() {
			ErrorKind::AlreadyExists => println!("File already exists : {}", file),
			_ => panic!("Error locating file : {} with error : {:?}", file, error),
		}
	}
}

pub fn create_workspace(working_dir: &String) {
	let mut tmp : String;
	for i in 0..7 {
		tmp = working_dir.clone();
		tmp.push_str("/");
		tmp.push_str(DIR_LIST[i]);
		create_dir(&*tmp);
	}
	create_file(&*working_dir, "Makefile", workspace_format::MAKEFILE);
	create_file(&*working_dir, "files.mk", workspace_format::FILEMK);
	tmp = working_dir.clone();
	tmp.push_str("/srcs/");
	create_file(&*tmp, "main.cpp", workspace_format::MAIN);
}

pub fn dir_exist(location: String) -> bool {
	let path = env::current_dir();
	let path = match path {
		Ok(_path) => _path,
		Err(error) => panic!("Error while exporting directory {:?}", error),
	};
	let ret : bool = match fs::metadata(path.as_path().join(location)) {
		Ok(_ok) => true,
		Err(_error) => false,
	};
	ret
}
