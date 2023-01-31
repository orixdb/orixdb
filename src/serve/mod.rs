use std::fs;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::cli;
use crate::basics::{ self, StoreType };

pub fn main(matches: &ArgMatches) -> std::process::ExitCode {

	//########## ----- PART 1: PRELIMINARY TASKS ----- ##########//
	//###########################################################//


	// --> Setting important variables
	//--------------------------------

	let conf = basics::get_conf(); // global settings
	let store_dir: PathBuf; // Store's directory
	let store:  basics::Store; // Store's manifest
	let api_port: u16; // Port number for client connections
	let api_port_scan: bool; // Port scanning toggle for API connection
	let cluster_port: u16; // Port number for cluster connections
	let cluster_port_scan: bool; // Port scanning toggle for API connection
	let verbose: bool; // Whether or not the terminal is verbose

	let mut store_item: PathBuf; // A `pathbuf` to index resources in the store
	let store_text_content: String; // A String to store their content


	// --> Checking and loading the store and its manifest
	//----------------------------------------------------

	// Checking if the supplied directory exists
	if matches.contains_id("folder") {
		store_dir = PathBuf::from(matches.get_one::<String>("folder").unwrap());

		if !store_dir.exists() {
			cli::red_err(
				"The path supplied doesn't lead to an existing directory.".to_owned()
			);
			return std::process::ExitCode::FAILURE;
		}
	}
	else { store_dir = PathBuf::from("."); }

	// Checking and loading the manifest
	store_item  = store_dir.clone();
	store_item.push("manifest.json");
	if !store_item.exists() {
		cli::red_err(
			"The store doesn't contain a manifest.".to_owned()
			+ " Did you supplied the right path ?"
		);
		return std::process::ExitCode::FAILURE;
	}

	let store_text_try = fs::read_to_string(store_item);
	if store_text_try.is_err() {
		cli::red_err(
			"Failed to read the manifest.\n".to_owned()
			+ "Exiting..."
		);
		return std::process::ExitCode::FAILURE;
	}
	store_text_content = store_text_try.unwrap();
	let store_manifest_try = serde_json::from_str(&*store_text_content);
	if store_manifest_try.is_err() {
		cli::red_err(
			"Failed to parse the manifest.".to_owned()
			+ " Please check if it contains a valid JSON text.\n"
			+ "Exiting..."
		);
		return std::process::ExitCode::FAILURE;
	}
	store = store_manifest_try.unwrap();

	// Checking if the store's type allows data serving
	if store.kind == StoreType::Backup || store.kind == StoreType::Archive {
		cli::red_err(
			"A backup or an archive store can't be served.".to_owned()
		);
		return std::process::ExitCode::FAILURE;
	}

	// Checking if the store's version is compliant
	// with the running OrixDB version
	if store.major < conf.major {
		cli::red_err(
			"The major version of the store is lower than".to_owned()
			+ " The current " + &*conf.display_name + " version."
			+ " Such a version difference is extremely dangerous for your data integrity !"
			+ " Please consider upgrading the store with the `upgrade` subcommand."
		);
		return std::process::ExitCode::FAILURE;
	}
	if store.major > conf.major {
		cli::red_err(
			"The major version of the store is higher than".to_owned()
			+ " The current " + &*conf.display_name + " version."
			+ " Such a version difference is extremely dangerous for your data integrity !"
			+ " Please consider updating your software."
		);
		return std::process::ExitCode::FAILURE;
	}
	if store.minor < conf.minor {
		cli::yellow_err(
			"The minor version of the store is lower than".to_owned()
			+ " The current " + &*conf.display_name + " version."
			+ " Version differences can have severe consequences on your data integrity !"
			+ " Please consider upgrading the store with the `upgrade` subcommand."
		);
	}
	else if store.minor > conf.minor {
		cli::yellow_err(
			"The minor version of the store is higher than".to_owned()
			+ " The current " + &*conf.display_name + " version."
			+ " Such a version difference is extremely dangerous for your data integrity !"
			+ " Please consider updating your software."
		);
		let pursue = inquire::Confirm::new("Continue anyway ?")
			.prompt().unwrap()
		;
		if !pursue { return std::process::ExitCode::SUCCESS; }
	}


	// --> Checking and loading the command line arguments
	//----------------------------------------------------

	// Setting the terminal verbosity
	verbose = *matches.get_one::<bool>("verbose").unwrap();

	let mut port_text: &String;
	let mut port_digest: (u16, bool);

	// Checking the supplied API port
	if matches.contains_id("api-port") {
		port_text = matches.get_one::<String>("api-port").unwrap();
		port_digest = basics::parse_port(port_text, "API");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		api_port = port_digest.0;
		api_port_scan = port_digest.1;
	}
	else {
		api_port = store.defaults.api_port;
		api_port_scan = store.defaults.api_scan;
	}

	// Checking the supplied cluster port
	if matches.contains_id("cluster-port") {
		port_text = matches.get_one::<String>("cluster-port").unwrap();
		port_digest = basics::parse_port(port_text, "cluster");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		cluster_port = port_digest.0;
		cluster_port_scan = port_digest.1;
	}
	else {
		cluster_port = store.defaults.cluster_port;
		cluster_port_scan = store.defaults.cluster_scan;
	}

	println!("{}", api_port);
	println!("{}", api_port_scan);
	println!("{}", cluster_port);
	println!("{}", cluster_port_scan);
	println!("{:?}", store_dir);
	println!("{}", verbose);
	return std::process::ExitCode::SUCCESS;
}