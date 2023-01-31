use std::collections::HashMap;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::cli;
use crate::basics::{ self, Instance, LogLevel, StoreType, Store };

fn check_id(id: &String) -> bool {
	if !id.chars().all(
		|c: char| {
			(c.is_ascii_alphabetic() && c.is_lowercase())
			|| c.is_ascii_digit()
			|| "-_".contains(c)
		}
	) {
		cli::red_err(
			"The store id must contain only lowercase\n".to_owned()
			+ "alphanumeric characters, dashes and underscores."
		);
		return false;
	}
	return true;
}

pub fn main(matches: &ArgMatches) -> std::process::ExitCode {
	let store_type_options = HashMap::from([
		("live", StoreType::Live),
		("lite", StoreType::Lite),
		("backup", StoreType::Backup),
		("archive", StoreType::Archive)
	]);
	let store_type_strings = store_type_options
		.keys().cloned().collect::<Vec<&str>>()
	;

	let log_level_options = HashMap::from([
		("off", LogLevel::Off),
		("minimal", LogLevel::Minimal),
		("normal", LogLevel::Normal),
		("detailed", LogLevel::Detailed)
	]);
	let log_level_strings = log_level_options
		.keys().cloned().collect::<Vec<&str>>()
	;

	let mut inst_path: PathBuf;
	let inst_dir: String;
	let inst_exists: bool;

	let conf = basics::get_conf();
	let mut store = Store {
		name: String::from(""),
		id: String::from(""),
		major: conf.major,
		minor: conf.minor,
		kind: StoreType::Live,
		ordering: false,
		checksumming: true,
		logging: LogLevel::Normal,
		defaults: Instance {
			verbosity: false,
			api_port: 7900,
			api_scan: false,
			cluster_port: 7979,
			cluster_scan: false
		}
	};

	println!("\
		░█████╗░██████╗░██╗██╗░░██╗██████╗░██████╗░\n\
		██╔══██╗██╔══██╗██║╚██╗██╔╝██╔══██╗██╔══██╗\n\
		██║░░██║██████╔╝██║░╚███╔╝░██║░░██║██████╦╝\n\
		██║░░██║██╔══██╗██║░██╔██╗░██║░░██║██╔══██╗\n\
		╚█████╔╝██║░░██║██║██╔╝╚██╗██████╔╝██████╦╝\n\
		░╚════╝░╚═╝░░╚═╝╚═╝╚═╝░░╚═╝╚═════╝░╚═════╝░\n\
	");
	println!("Welcome ! You are going to create a new OrixDB store.\n");

	if matches.contains_id("directory") {
		let directory = matches.get_one::<String>("directory")
			.unwrap()
		;
		let mut inst_temp = PathBuf::from(directory);
		if inst_temp.is_file() {
			cli::red_err(
				"The installation path resolves to a file.\n".to_owned()
				+ "A new store can't be set in a file but in a directory."
			);
			return std::process::ExitCode::FAILURE;
		}
		if inst_temp.is_dir() {
			let is_empty = inst_temp.read_dir().unwrap().next().is_none();
			if !is_empty {
				cli::red_err(
					"The installation directory is not empty.\n".to_owned()
					+ "Then a new store can't be set there."
				);
				return std::process::ExitCode::FAILURE;
			}
			inst_path = inst_temp.canonicalize().unwrap();
			inst_dir = inst_path.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			inst_exists = true;
		}
		else {
			if inst_temp.is_relative() {
				inst_temp = PathBuf::from("./".to_owned() + directory);
			}

			let temp_parent = inst_temp.parent().unwrap();
			if temp_parent.is_file() {
				cli::red_err(
					"The path: \"".to_owned()
					+ temp_parent.to_str().unwrap()
					+ "\" resolves to a file.\n"
					+ "Then a new store can't be set in it."
				);
				return std::process::ExitCode::FAILURE;
			}

			inst_path = temp_parent.to_path_buf();
			inst_dir = inst_temp.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			inst_path.push(&inst_dir);
			inst_exists = false;
		}
	}
	else {
		inst_path = std::env::current_dir().unwrap();
		let is_empty = inst_path.read_dir().unwrap().next().is_none();
		if !is_empty {
			cli::red_err(
				"The current directory is not empty.\n".to_owned()
				+ "Then a new store can't be set here.\n"
				+ "You can specify another installation directory as argument."
			);
			return std::process::ExitCode::FAILURE;
		}
		inst_dir = inst_path.file_name().unwrap()
			.to_os_string().into_string().unwrap()
		;
		inst_exists = true;
	}

	if matches.contains_id("id") {
		store.id = matches.get_one::<String>("id").unwrap().to_string();
		if !check_id(&store.id) { return std::process::ExitCode::FAILURE }
	}

	if matches.contains_id("type") {
		let store_type = matches.get_one::<String>("type")
			.unwrap().to_string()
			;
		if !store_type_options.contains_key(&*store_type) {
			cli::red_err(
				"The store type must have one of the ".to_owned()
				+ "authorized values.\n(Try: `orixdb help create` to know more...)"
			);
			return std::process::ExitCode::FAILURE;
		}
		store.kind = store_type_options[&*store_type];
	}

	if matches.contains_id("logging") {
		let store_logging = matches.get_one::<String>("logging")
			.unwrap().to_string()
		;
		if !log_level_options.contains_key(&*store_logging) {
			cli::red_err(
				"The store logging mode must have one of the ".to_owned()
				+ "authorized values.\n(Try: `orixdb help create` to know more...)"
			);
			return std::process::ExitCode::FAILURE;
		}
		store.logging = log_level_options[&*store_logging];
	}

	println!("✔ Store location: \x1b[2m\x1b[36m{}\x1b[0m", inst_path.display());

	if matches.contains_id("name") {
		store.name = matches.get_one::<String>("name").unwrap().to_string();
			println!("✔ Store name: {}", store.name);
	}
	else {
		store.name = inquire::Text::new("Store name: ")
			.with_default(&*inst_dir).prompt().unwrap()
		;
	}

	if matches.contains_id("id") {
		println!("✔ Store id: {}", store.id);
	}
	else {
		store.id = inquire::Text::new("Store id: ")
			.with_default(&*slug::slugify(store.name.clone()))
			.prompt().unwrap()
		;
		if !check_id(&store.id) { return std::process::ExitCode::FAILURE }
	}

	if matches.contains_id("type") {
		println!("✔ Store type: {:?}", store.kind);
	}
	else {
		let store_type = inquire::Select::new(
			"Store type:", store_type_strings
		)
			.prompt().unwrap()
		;
		store.kind = store_type_options[store_type];
	}

	if *matches.get_one::<bool>("ordering").unwrap() {
		store.ordering = true;
		println!("✔ Automatic data ordering: Yes");
	}
	else {
		store.ordering = inquire::Confirm::new("Automatic data ordering ?")
			.with_default(false).prompt().unwrap()
		;
	}

	if *matches.get_one::<bool>("checksumming").unwrap() {
		store.checksumming = true;
		println!("✔ Data checksumming: Yes");
	}
	else {
		store.checksumming = inquire::Confirm::new("Data checksumming ?")
			.with_default(true).prompt().unwrap()
		;
	}

	if matches.contains_id("logging") {
		println!("✔ Store logging mode: {:?}", store.logging);
	}
	else {
		let store_logging = inquire::Select::new(
			"Store logging mode:", log_level_strings
		)
			.prompt().unwrap()
		;
		store.logging = log_level_options[store_logging];
	}

	println!(
		"\n{}",
		"Defaults settings for each run of OrixDB on this store:\n".to_owned()
		+ "\x1b[36mVerbosity\x1b[0m: \x1b[34m\x1b[1mNo;\x1b[0m "
		+ "\x1b[36mAPI port\x1b[0m: \x1b[34m\x1b[1m7900...;\x1b[0m "
		+ "\x1b[36mCluster port\x1b[0m: \x1b[34m\x1b[1m7979...;\x1b[0m"
	);
	let change_defaults = inquire::Confirm::new(
		"Do you want to change them ?"
	).with_default(false).prompt().unwrap();
	if change_defaults {
		store.defaults.verbosity = inquire::Confirm::new("Verbose terminal ?")
			.with_default(false).prompt().unwrap()
		;

		let mut port_text: String;
		let mut port_digest: (u16, bool);

		port_text = inquire::Text::new("Default API port:")
			.with_help_message("\
				You can add an ellipsis (Ex: 5500...) to allow port scanning, starting\n\
				from the given number, in case the the port is not free.\
			").with_default("7900...").prompt().unwrap()
		;
		port_digest = basics::parse_port(&port_text, "API");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		store.defaults.api_port = port_digest.0;
		store.defaults.api_scan = port_digest.1;

		port_text = inquire::Text::new("Default cluster port:")
			.with_help_message("\
				You can add an ellipsis (Ex: 5500...) to allow port scanning, starting\n\
				from the given number, in case the the port is not free.\
			").with_default("7979...").prompt().unwrap()
		;
		port_digest = basics::parse_port(&port_text, "cluster");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		store.defaults.cluster_port = port_digest.0;
		store.defaults.cluster_scan = port_digest.1;
	}

	let mut try_fs: std::io::Result<()>;
	let mut try_fs_file: std::io::Result<std::fs::File>;

	fn more_errors() {
		cli::red_err(
			"Failed to create some resources.\n".to_owned()
				+ "Do you have a write permission in the store directory ?\n"
				+ "Exiting..."
		);
	}

	if !inst_exists {
		try_fs = std::fs::create_dir_all(&inst_path);
		if try_fs.is_err() {
			cli::red_err(
				"Failed to create the store's directory.\n".to_owned()
				+ "Do you have a write permission in the parent directory ?\n"
				+ "Exiting..."
			);
			return std::process::ExitCode::FAILURE;
		}
	}

	let store_text = serde_json::to_string_pretty(&store).unwrap();
	let mut store_manifest = inst_path.clone();
	store_manifest.push("manifest.json");
	try_fs = std::fs::write(store_manifest, store_text);
	if try_fs.is_err() {
		cli::red_err(
			"Failed to create the manifest.\n".to_owned()
			+ "Do you have a write permission in the store directory ?\n"
			+ "Exiting..."
		);
		return std::process::ExitCode::FAILURE;
	}

	let mut store_singles = inst_path.clone();
	store_singles.push("singletons");
	try_fs = std::fs::create_dir_all(&store_singles);
	if try_fs.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut singles_index = store_singles.clone();
	singles_index.push("rixindex");
	try_fs_file = std::fs::File::create(singles_index);
	if try_fs_file.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut store_colls = inst_path.clone();
	store_colls.push("collections");
	try_fs = std::fs::create_dir_all(&store_colls);
	if try_fs.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut colls_index = store_colls;
	colls_index.push("rixindex");
	try_fs_file = std::fs::File::create(colls_index);
	if try_fs_file.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut store_checks = inst_path.clone();
	store_checks.push("checksums");
	try_fs = std::fs::create_dir_all(&store_checks);
	if try_fs.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut store_logs = inst_path.clone();
	store_logs.push("logs");
	try_fs = std::fs::create_dir_all(&store_logs);
	if try_fs.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	let mut store_temp = inst_path.clone();
	store_temp.push("tmp");
	try_fs = std::fs::create_dir_all(&store_temp);
	if try_fs.is_err() {
		more_errors();
		return std::process::ExitCode::FAILURE;
	}

	return std::process::ExitCode::SUCCESS;
}