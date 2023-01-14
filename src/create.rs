use std::collections::HashMap;
use std::path::PathBuf;

use clap::ArgMatches;
use inquire::{Select, Text};
use slug::slugify;

use crate::basics;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum LogLevel {
	Off,
	Minimal,
	Normal,
	Detailed
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum StoreType {
	Live,
	Lite,
	Backup,
	Archive
}

struct Instance {
	buffering: bool,
	api_port: u16,
	cluster_port: u16
}

struct Store {
	name: String,
	slug: String,
	kind: StoreType,
	ordered: bool,
	checksumming: bool,
	logging: LogLevel,
	defaults: Instance
}

fn check_slug(slug: &String) -> bool {
	if !slug.chars().all(
		|c: char| {
			(c.is_ascii_alphabetic() && c.is_lowercase())
			|| c.is_ascii_digit()
			|| "-_".contains(c)
		}
	) {
		basics::red_err(
			"The store slug must contain only lowercase\n".to_owned()
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

	let mut dest_path: PathBuf;
	let mut dest_parent: PathBuf = PathBuf::new();
	let dest_folder: String;
	let dest_exists: bool;

	let mut store = Store{
		name: String::from(""),
		slug: String::from(""),
		kind: StoreType::Live,
		ordered: false,
		checksumming: true,
		logging: LogLevel::Normal,
		defaults: Instance {
			buffering: false,
			api_port: 7979,
			cluster_port: 7900
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
		let mut dest_temp = PathBuf::from(folder);
		if dest_temp.is_file() {
			basics::red_err(
				"The destination path resolves to a file.\n".to_owned()
				+ "A new store can't be set in a file but in a directory."
			);
			return std::process::ExitCode::FAILURE;
		}
		if dest_temp.is_dir() {
			let is_empty = dest_temp.read_dir().unwrap().next().is_none();
			if !is_empty {
				basics::red_err(
					"The destination folder is not empty.\n".to_owned()
					+ "Then a new store can't be set there."
				);
				return std::process::ExitCode::FAILURE;
			}
			dest_path = dest_temp.canonicalize().unwrap();
			dest_folder = dest_path.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			dest_exists = true;
		}
		else {
			if dest_temp.is_relative() {
				dest_temp = PathBuf::from("./".to_owned() + folder);
			}

			let temp_parent = dest_temp.parent().unwrap();
			if !temp_parent.exists() {
				basics::red_err(
					"The folder: \"".to_owned()
					+ temp_parent.to_str().unwrap()
					+ "\" does not exist.\n"
					+ "Then a new store can't be set in it."
				);
				return std::process::ExitCode::FAILURE;
			}
			if temp_parent.is_file() {
				basics::red_err(
					"The path: \"".to_owned()
					+ temp_parent.to_str().unwrap()
					+ "\" resolves to a file.\n"
					+ "Then a new store can't be set in it."
				);
				return std::process::ExitCode::FAILURE;
			}

			dest_parent = temp_parent.canonicalize().unwrap();
			dest_path = dest_parent.clone();
			dest_folder = dest_temp.file_name().unwrap()
				.to_os_string().into_string().unwrap()
			;
			dest_path.push(&dest_folder);
			dest_exists = false;
		}
	}
	else {
		dest_path = std::env::current_dir().unwrap();
		let is_empty = dest_path.read_dir().unwrap().next().is_none();
		if !is_empty {
			basics::red_err(
				"The current folder is not empty.\n".to_owned()
				+ "Then a new store can't be set here."
			);
			return std::process::ExitCode::FAILURE;
		}
		dest_folder = dest_path.file_name().unwrap()
			.to_os_string().into_string().unwrap()
		;
		dest_exists = true;
	}

	if matches.contains_id("slug") {
		store.slug = matches.get_one::<String>("slug").unwrap().to_string();
		if !check_slug(&store.slug) { return std::process::ExitCode::FAILURE }
	}

	if matches.contains_id("type") {
		let store_type = matches.get_one::<String>("type")
			.unwrap().to_string()
		;
		if !store_type_options.contains_key(&*store_type) {
			basics::red_err(
				"The store type must have one of the ".to_owned()
					+ "authorized values.\n(Try: `orixdb help create`)"
			);
			return std::process::ExitCode::FAILURE;
		}
		store.kind = store_type_options[&*store_type];
	}

	println!("✔ Store location: \x1b[2m\x1b[36m{}\x1b[0m", dest_path.display());

	if matches.contains_id("name") {
		store.name = matches.get_one::<String>("name").unwrap().to_string();
		println!("✔ Store name: {}", store.name);
	}
	else {
		store.name = Text::new("Store name: ")
			.with_default(&*dest_folder).prompt().unwrap()
		;
	}

	if matches.contains_id("slug") {
		println!("✔ Store slug: {}", store.slug);
	}
	else {
		store.slug = Text::new("Store slug: ")
			.with_default(&*slugify(store.name.clone()))
			.prompt().unwrap()
		;
		if !check_slug(&store.slug) { return std::process::ExitCode::FAILURE }
	}

	if matches.contains_id("type") {
		println!("✔ Store type: {:?}", store.kind);
	}
	else {
		let store_type = Select::new("Store type:", store_type_strings)
			.prompt().unwrap()
		;
		store.kind = store_type_options[store_type];
	}

	// if matches.contains_id("logging") {
	// 	store.slug = matches.get_one::<String>("logging").unwrap().clone();
	// 	println!("✔ Store slug: {}", store.slug);
	// }
	// else {
	// 	let store_type = Select::new("Store type:", log_level_strings)
	// 		.prompt().unwrap()
	// 		;
	// 	store.logging = log_level_options[store_type];
	// }

	println!();
	println!("path: {:#?}", dest_path);
	println!("folder: {:#?}", dest_folder);
	println!("exists: {:#?}", dest_exists);
	if !dest_exists { println!("parent: {:#?}", dest_parent) };
	println!("name: {:#?}", store.name);
	println!("slug: {:#?}", store.slug);
	println!("kind: {:#?}", store.kind);
	println!("ord: {:#?}", store.ordered);
	println!("check: {:#?}", store.checksumming);
	println!("log: {:#?}", store.logging);
	println!("def.buff: {:#?}", store.defaults.buffering);
	println!("def.api: {:#?}", store.defaults.api_port);
	println!("def.clu: {:#?}", store.defaults.cluster_port);

	return std::process::ExitCode::SUCCESS;
}