use std::path::PathBuf;

use clap::ArgMatches;

use crate::cli;
use crate::basics;

pub fn main(matches: &ArgMatches) -> std::process::ExitCode {

	//########## ----- PART 1: PRELIMINARY TASKS ----- ##########//
	//###########################################################//


	// --> Setting important variables
	//--------------------------------

	let store_dir; // Store's directory
	// let store; // Store's manifest'
	let api_port; // Port number for client connections
	let api_port_scan; // Port scanning toggle for API connection
	let cluster_port; // Port number for cluster connections
	let cluster_port_scan; // Port scanning toggle for API connection
	let verbose; // Whether or not the terminal is verbose


	// --> Checking the command line arguments
	//----------------------------------------

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

	// Setting the terminal verbosity
	verbose = *matches.get_one::<bool>("verbose").unwrap();

	let mut port_text;
	let mut port_digest;

	// Checking the supplied API port
	if matches.contains_id("api-port") {
		port_text = matches.get_one::<String>("api-port").unwrap();
		port_digest = basics::parse_port(port_text, "API");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		api_port = port_digest.0;
		api_port_scan = port_digest.1;
	}

	// Checking the supplied cluster port
	if matches.contains_id("cluster-port") {
		port_text = matches.get_one::<String>("cluster-port").unwrap();
		port_digest = basics::parse_port(port_text, "cluster");
		if port_digest.0 == 0 { return std::process::ExitCode::FAILURE; }
		cluster_port = port_digest.0;
		cluster_port_scan = port_digest.1;
	}

	// Check and load store
	// Check version
	// Check type
	return std::process::ExitCode::SUCCESS;
}