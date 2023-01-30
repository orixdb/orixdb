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
	let inst_folder: String;
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

	if matches.contains_id("folder") {
		let folder = matches.get_one::<String>("folder")
			.unwrap()
		;
		let mut inst_temp = PathBuf::from(folder);
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
					"The installation folder is not empty.\n".to_owned()
					+ "Then a new store can't be set there."
				);
				return std::process::ExitCode::FAILURE;
			}
			inst_path = inst_temp.canonicalize().unwrap();
			inst_folder = inst_path.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			inst_exists = true;
		}
		else {
			if inst_temp.is_relative() {
				inst_temp = PathBuf::from("./".to_owned() + folder);
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
			inst_folder = inst_temp.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			inst_path.push(&inst_folder);
			inst_exists = false;
		}
	}
	else {
		inst_path = std::env::current_dir().unwrap();
		let is_empty = inst_path.read_dir().unwrap().next().is_none();
		if !is_empty {
			cli::red_err(
				"The current folder is not empty.\n".to_owned()
				+ "Then a new store can't be set here.\n"
				+ "You can specify another installation folder as argument."
			);
			return std::process::ExitCode::FAILURE;
		}
		inst_folder = inst_path.file_name().unwrap()
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
			.with_default(&*inst_folder).prompt().unwrap()
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
		"Defaults settings for each run OrixDB on this store:\n".to_owned()
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

		let mut number;
		let mut ellipsis;
		let mut num_try;
		let mut test_0;

		let api_port = inquire::Text::new("Default API port:")
			.with_help_message("\
				You can add an ellipsis (Ex: 5500...) to allow port scanning, starting\n\
				from the given number, in case the the port is not free.\
			").with_default("7900...").prompt().unwrap()
		;
		if api_port.find(".").is_some() {
			(number, ellipsis) = api_port.split_once(".").unwrap();
			if
				ellipsis.len() < 1 ||
				! ellipsis.chars().all(|c: char| c == '.')
			{
				cli::red_err(
					"The ellipsis must contain two or more periods.\n".to_owned()
					+ "And nothing else. (Ex: 5500...)"
				);
				return std::process::ExitCode::FAILURE;
			}
			store.defaults.api_scan = true;
		}
		else {
			number = &*api_port;
		}
		num_try = number.parse::<u16>();
		test_0 = num_try.clone();
		if num_try.is_err() || test_0.unwrap() == 0 {
			cli::red_err(
				"The API port must be a valid number between 1 and 65535, ".to_owned()
				+ "with an optional ellipsis at the end. (Ex: 5500...)"
			);
			return std::process::ExitCode::FAILURE;
		}
		store.defaults.api_port = num_try.unwrap();

		let cluster_port = inquire::Text::new("Default cluster port:")
			.with_help_message("\
				You can add an ellipsis (Ex: 5500...) to allow port scanning, starting\n\
				from the given number, in case the the port is not free.\
			").with_default("7979...").prompt().unwrap()
			;
		if cluster_port.find(".").is_some() {
			(number, ellipsis) = cluster_port.split_once(".").unwrap();
			if
				ellipsis.len() < 1 ||
				! ellipsis.chars().all(|c: char| c == '.')
			{
				cli::red_err(
					"The ellipsis must contain two or more periods.\n".to_owned()
						+ "And nothing else. (Ex: 5500...)"
				);
				return std::process::ExitCode::FAILURE;
			}
			store.defaults.cluster_scan = true;
		}
		else {
			number = &*cluster_port;
		}
		num_try = number.parse::<u16>();
		test_0 = num_try.clone();
		if num_try.is_err() || test_0.unwrap() == 0 {
			cli::red_err(
				"The cluster port must be a valid number between 1 and 65535, ".to_owned()
				+ "with an optional ellipsis at the end. (Ex: 5500...)"
			);
			return std::process::ExitCode::FAILURE;
		}
		store.defaults.cluster_port = num_try.unwrap();
	}

	if !inst_exists {
		std::fs::create_dir_all(&inst_path).unwrap();
	}

	let store_text = serde_json::to_string_pretty(&store).unwrap();
	let mut store_manifest = inst_path.clone();
	store_manifest.push("manifest.json");
	std::fs::write(store_manifest, store_text).unwrap();

	let mut store_singles = inst_path.clone();
	store_singles.push("singletons");
	std::fs::create_dir_all(&store_singles).unwrap();

	let mut singles_index = store_singles.clone();
	singles_index.push("rixindex");
	std::fs::File::create(singles_index).unwrap();

	let mut store_colls = inst_path.clone();
	store_colls.push("collections");
	std::fs::create_dir_all(&store_colls).unwrap();

	let mut colls_index = store_colls;
	colls_index.push("rixindex");
	std::fs::File::create(colls_index).unwrap();

	let mut store_checks = inst_path.clone();
	store_checks.push("checksums");
	std::fs::create_dir_all(&store_checks).unwrap();

	let mut store_logs = inst_path.clone();
	store_logs.push("logs");
	std::fs::create_dir_all(&store_logs).unwrap();

	let mut store_temp = inst_path.clone();
	store_temp.push("tmp");
	std::fs::create_dir_all(&store_temp).unwrap();

	return std::process::ExitCode::SUCCESS;
}