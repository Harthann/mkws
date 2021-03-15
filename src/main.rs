use std::env;

mod file_dir;
mod file_format;
mod add;

#[derive(Clone)]
#[derive(Debug)]
struct SFile {
	location: String,
	base_dir: String,
	content: String,
	file_name: String,
	ftype: String,
}

fn create_class(class: &String, dir: &str, content: &[u8], ftype: &str) -> SFile {
	let mut file = SFile {
		location: "".to_string(),
		base_dir: dir.to_string(),
		content: std::str::from_utf8(content)
									.expect("Error converting u8 to str")
									.to_string(),
		file_name: class.clone(),
		ftype: "class source".to_string(),
	};
	if ftype == "class source" {
		file.file_name.push_str(".cpp");
	} else {
		file.file_name.push_str(".hpp");
	}
	let mut pattern_offest = file.content.find("__name__");
	while pattern_offest.is_some() {
		let idx = pattern_offest.unwrap();
		file.content.replace_range(idx..(idx + 8), &*class);
		pattern_offest = file.content.find("__name__");
	}
	pattern_offest = file.content.find("__NAME__");
	while pattern_offest.is_some() {
		let idx = pattern_offest.unwrap();
		file.content.replace_range(idx..(idx + 8), &*class.to_uppercase());
		pattern_offest = file.content.find("__NAME__");
	}
	file
}

fn print_options(args: &String) {
	println!("Wrong options : {}", args);
	println!("Files don't need extension, it will automatically consider c++ project extension");
	println!("Format: -ws [DIR] : create workspace with name DIR in current directory");
	println!("\t-s [FILE] : create source file in srcs directory");
	println!("\t-d [DIR]  : add subdirectory for source file (subdirectory lie under srcs directory and affect all source linked)");
	println!("\t-c [FILE] : create source and header file for class given");
	println!("\t-h [FILE] : create header file");
	println!("\t-t [FILE] : create template file");
	println!("\t-i [FILE] : create interface class");
}

fn options(args: &mut Vec<String>) -> Result<Vec<SFile>, &'static str> {
	let mut list : Vec<SFile> = Vec::new();
	let mut base_location = String::from("");
	for i in (1..args.len()).step_by(2) {
		let mut files =  SFile {
			file_name : "".to_string(),
			base_dir : "".to_string(),
			location : base_location.clone(),
			content : "".to_string(),
			ftype : "ignore".to_string(),
		};
		if i + 1 >= args.len() && args[i].as_str() != "--help" {
			println!("Missing argument for option : {}", args[i]);
			return Err("Aborting");
		}
		match args[i].as_str() {
			"-s" => {
				files.file_name = args[i + 1].clone();
				files.file_name.push_str(".cpp");
				files.base_dir = "srcs/".to_string();
				files.content = "".to_string();
				files.ftype = "source".to_string();
				add::link_file(&mut files.file_name.clone(), &"SRC_FILE= ".to_string());
			},
			"-d" => {
				for j in 0..list.len() {
					if list[j].ftype == "source" {
						list[j].location = args[i + 1].clone();
					}
				}
				base_location = args[i + 1].clone();
			},
			"-c" => {
				list.push(create_class(&args[i + 1], "srcs/classes", file_format::CLASS_CPP, "class source"));
				list.push(create_class(&args[i + 1], "include/classes", file_format::CLASS_HPP, "class header"));
				add::link_file(&mut args[i + 1], &"CLASSES= ".to_string());
			},
			"-t" => {
				let mut tmp = args[i + 1].clone();
				tmp.push_str(".hpp");
				list.push(create_class(&args[i + 1], "include/template", file_format::HEADER_HPP, "template header"));
				add::link_file(&mut tmp.clone(), &"TEMPLATES= ".to_string());
			},
			"-h" => {
				let mut tmp = args[i + 1].clone();
				tmp.push_str(".hpp");
				list.push(create_class(&args[i + 1], "include/", file_format::HEADER_HPP, "header"));
				add::link_file(&mut tmp.clone(), &"HEADERS= ".to_string());
			},
			"-i" => {
				let mut tmp = args[i + 1].clone();
				tmp.push_str(".hpp");
				list.push(create_class(&args[i + 1], "include/classes/", file_format::INTERFACE_HPP, "interface"));
				add::link_file(&mut tmp.clone(), &"INTERFACES= ".to_string());
			},
			"-ws" => {
				file_dir::create_dir(&*args[i + 1]);
				file_dir::create_workspace(&args[i + 1]);
			},
			"--help" => print_options(&args[i]),
			_ => print_options(&args[i]),
		}
		list.push(files.clone());
	}
	Ok(list)
}

fn main() {
	let mut args : Vec<String> = env::args().collect();
	if args.len() == 1 {
		file_dir::create_workspace(&".".to_string());
	}
	else {
		let list = options(&mut args);
		if list.is_ok() {
			let list = list.unwrap();
			for mut files in list {
			
				if files.ftype != "ignore" {
					files.base_dir.push_str(&*files.location);
					if !file_dir::dir_exist(files.base_dir.clone()) {
						file_dir::create_dir(&*files.base_dir);
					}
					file_dir::create_file(&*files.base_dir, &*files.file_name, files.content.as_bytes());
				}
			}
		} else {
			println!("{}", list.unwrap_err());
		}
	}
}
