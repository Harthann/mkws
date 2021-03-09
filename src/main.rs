use std::io::ErrorKind;
use std::env;
use std::fs;

mod filemk;
mod makefile;

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

fn create_workspace() {
	create_dir("include");
	create_dir("srcs");
	create_dir("lib");
	create_dir("objs");
	println!("{}", makefile::MAKEFILE);
	println!("{}", filemk::FILEMK);
}

fn add(args: &Vec<String>) {
	for i in 0..args.len() {
		println!("Entered add function {:?}", args[i]);
	}
}

fn main() {
    let args : Vec<String> = env::args().collect();
	if args.len() == 1 {
		create_workspace();
	}
	else {
		add(&args);
	}
}
