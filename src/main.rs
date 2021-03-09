use std::io::ErrorKind;
use std::env;
use std::fs;
use std::io::Write;

mod makefile;
mod add;

fn create_dir(dir: &str) {
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

fn create_file(location: &str, file: &str, content: &[u8]) {
	let path = match env::current_dir() {
		Ok(_path) => _path,
		Err(error) => panic!("Error while exporting directory {:?}", error),
	};
	let file = fs::OpenOptions::new().write(true)
									.create_new(true)
									.open(path.as_path().join(location).join(file));
	let mut file = match file {
		Ok(_file) => _file,
		Err(error) => panic!("File already exists : {:?}", error),
	};
	file.write_all(content).expect("Couldn't write in file");
}

fn create_workspace() {
	create_dir("test/include");
	create_dir("test/include/template");
	create_dir("test/include/classes");
	create_dir("test/srcs");
	create_dir("test/lib");
	create_dir("test/objs");
	create_file("test/", "Makefile", makefile::MAKEFILE);
	create_file("test/", "file.mk", makefile::FILEMK);
}

fn options(args: &Vec<String>) {
	if args[1] == "add" {
		add::add(args);
	}
	else {
		println!("Wrong options entered");
	}
}

fn main() {
    let args : Vec<String> = env::args().collect();
	if args.len() == 1 {
		create_workspace();
	}
	else {
		options(&args);
	}
}
