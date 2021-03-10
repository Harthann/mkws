use std::io::ErrorKind;
use std::env;
use std::fs;
use std::io::Write;

mod makefile;
mod add;
mod file_format;

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
		Err(error) => {
			println!("File already exists : {:?}", error);
			file
		}
	};
	file.write_all(content).expect("Couldn't write in file");
}

fn create_workspace() {
	create_dir("include");
	create_dir("include/template");
	create_dir("include/classes");
	create_dir("srcs");
	create_dir("lib");
	create_dir("objs");
	create_file("", "Makefile", makefile::MAKEFILE);
	create_file("", "files.mk", makefile::FILEMK);
	create_file("srcs", "main.cpp", file_format::MAIN);
}

fn options(args: &mut Vec<String>) {
	let mut location = String::from("");
	let mut base_dir = String::from("");
	let mut base_content = String::from("");
	let mut file_name = String::from("");
	println!("{}", args.len());
	for i in (1..args.len()).step_by(2) {
		match args[i].as_str() {
			"-s" => {
				add::link_file(&mut args[i + 1], &"SRC_FILE= ".to_string());
				file_name = args[i + 1].clone();
				base_dir = "srcs/".to_string();
			},
			"-d" => location = args[i + 1].clone(),
			_ => println!("{}: Not a valid options pattern", args[i]),
			// "-c" => ,
			// "-i" => ,
			// "-t" => ,
			// "-h" => ,
		}
	}
	base_dir.push_str(&*location);
	create_file(&*base_dir, &*file_name, base_content.as_bytes());
	println!("{} {}", base_dir, file_name);
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
	if args.len() == 1 {
		create_workspace();
	}
	else {
		options(&mut args);
	}
}
