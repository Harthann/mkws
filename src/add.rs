fn add_src() {
	println!("Adding src");
}

pub fn add(args: &Vec<String>) {
	match args[2].as_str() {
		"class" => println!("Adding class"),
		"interface" => println!("Adding interface"),
		"template" => println!("Adding template"),
		"src" => add_src(),
		"header" => println!("Adding header"),
		_ => println!("Wrong file type")
	};
}