use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::{self, Read};
use std::string::FromUtf8Error;

use clap::ArgMatches;
use byteorder::{ ReadBytesExt, BigEndian };

use crate::cli;
use crate::basics::{ self, StoreType };

#[derive(Debug)]
#[derive(Clone)]
struct FileMeta {
	size: u64,
	reads: u16,
	writing: bool,
	holes: HashMap<
		u64, // index
		u64 // length
	>
}

#[derive(Debug)]
// #[derive(Clone)]
struct DataType {
	value: u8
}

#[derive(Debug)]
#[derive(Clone)]
struct SingletonMeta {
	name: String,
	data_type: u8,
	file: String,
	index: u64,
	data_length: u64
}

#[derive(Debug)]
// #[derive(Clone)]
struct CollectionMeta {
	data_type: u8,
	file: String,
	index: u64,
	data_length: u64
}

fn store_read_err(store_item: PathBuf) -> String {
	return format!(
		"Failed to load the content of: {:?}",
		store_item
	);
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
pub fn main(matches: &ArgMatches) -> std::process::ExitCode {

	//########## ----- PART 1: PRELIMINARY TASKS ----- ##########//
	//###########################################################//


	// --> Setting important variables
	// -------------------------------

	let conf = basics::get_conf(); // Global settings
	let store_dir: PathBuf; // Store's directory
	let store:  basics::Store; // Store's manifest
	let api_port: u16; // Port number for client connections
	let api_port_scan: bool; // Port scanning toggle for API connection
	let cluster_port: u16; // Port number for cluster connections
	let cluster_port_scan: bool; // Port scanning toggle for API connection
	let verbose: bool; // Whether or not the terminal is verbose

	// Map relating each singleton id to its metadata
	let mut singletons = HashMap::<String, SingletonMeta>::new();
	// Map relating each singleton file name to its metadata
	let mut singleton_files = HashMap::<String, FileMeta>::new();

	// Map relating each Collection to its metadata
	let mut collections_list = HashMap::<String, String>::new();
	// Map relating each collection item id to its location
	let mut collections = HashMap::<
		String, (DataType, CollectionMeta)
	>::new();
	// Map relating each hole location in collections, to its size
	// let mut collections_files = HashMap::<
	// 	String, HashMap<(String, u64), u64>
	// >::new();

	let mut store_item: PathBuf; // A `pathbuf` to index resources in the store
	let store_text_content: String; // A String to store their content
	// let mut store_file_name: String; // A string store temporarily file names
	// let mut store_file_length: u64;
	let mut store_file_handle: io::BufReader<fs::File>;
	let mut store_bin_content = Vec::<u8>::new();
	let mut io_read_try: io::Result<usize>;
	let mut bo_read8_try: io::Result<u8>;
	let mut bo_read_try: io::Result<u64>;
	let mut store_item_number: u64;
	let mut store_str_length: u8;
	let mut store_id_str: String;
	let mut store_str_draft: Vec<u8>;
	let mut store_str_parse: Result<String, FromUtf8Error>;
	let mut store_singleton_meta = SingletonMeta {
		name: String::new(),
		data_type: 0u8,
		file: String::new(),
		index: 0,
		data_length: 0
	};


	// --> Checking and loading the store and its manifest
	// ---------------------------------------------------

	// Checking if the supplied directory exists
	if matches.contains_id("directory") {
		store_dir = PathBuf::from(matches.get_one::<String>("directory").unwrap());

		if !store_dir.exists() {
			cli::red_err(
				"The path supplied doesn't lead to an existing directory.".to_owned()
			);
			return std::process::ExitCode::FAILURE;
		}
	}
	else { store_dir = PathBuf::from(".").canonicalize().unwrap(); }

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
	// ---------------------------------------------------

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



	//########## ----- PART 2: LOADING THE STORE'S METADATA ----- ##########//
	//######################################################################//


	// --> Loading the index and files of the singletons
	// -------------------------------------------------

	store_item = store_dir.clone();
	store_item.push("singletons/rixindex");
	if !store_item.exists() {
		cli::red_err(
			"The file: \"".to_owned()
			+ store_item.to_str().unwrap()
			+ "\" was not found !"
		);
		return std::process::ExitCode::FAILURE;
	}
	store_file_handle = io::BufReader::new(fs::File::open(&store_item)
		.unwrap()
	);
	store_bin_content.resize(12, 0);

	// Loading the files list for singletons
	bo_read_try = store_file_handle.read_u64::<BigEndian>();
	if bo_read_try.is_err() {
		cli::red_err(store_read_err(store_item));
		return std::process::ExitCode::FAILURE;
	}
	store_item_number = bo_read_try.unwrap();
	for i in 0..store_item_number {
		io_read_try = store_file_handle.read(&mut store_bin_content[0..12]);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..12 as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_id_str = store_str_parse.unwrap();

		bo_read_try = store_file_handle.read_u64::<BigEndian>();
		if bo_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}

		singleton_files.insert(store_id_str, FileMeta {
			size: bo_read_try.unwrap(),
			reads: 0,
			writing: false,
			holes: HashMap::<u64, u64>::new()
		});
	}

	// Loading the data index for singletons
	bo_read_try = store_file_handle.read_u64::<BigEndian>();
	if bo_read_try.is_err() {
		cli::red_err(store_read_err(store_item));
		return std::process::ExitCode::FAILURE;
	}
	store_item_number = bo_read_try.unwrap();
	for i in 0..store_item_number {
		bo_read8_try = store_file_handle.read_u8();
		if bo_read8_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_length = bo_read8_try.unwrap();

		if store_bin_content.len() < store_str_length as usize {
			store_bin_content.resize(store_str_length as usize, 0);
		}
		io_read_try = store_file_handle.read(
			&mut store_bin_content[0..store_str_length as usize]
		);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..store_str_length as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_singleton_meta.name = store_str_parse.unwrap();

		io_read_try = store_file_handle.read(&mut store_bin_content[0..12]);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..12 as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_id_str = store_str_parse.unwrap();

		bo_read8_try = store_file_handle.read_u8();
		if bo_read8_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_singleton_meta.data_type = bo_read8_try.unwrap();

		io_read_try = store_file_handle.read(&mut store_bin_content[0..12]);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..12 as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_singleton_meta.file = store_str_parse.unwrap();

		bo_read_try = store_file_handle.read_u64::<BigEndian>();
		if bo_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_singleton_meta.index = bo_read_try.unwrap();

		bo_read_try = store_file_handle.read_u64::<BigEndian>();
		if bo_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_singleton_meta.data_length = bo_read_try.unwrap();

		singletons.insert(store_id_str, store_singleton_meta.clone());
	}


	// --> Loading the indices and files of the collections
	// ----------------------------------------------------

	store_item = store_dir.clone();
	store_item.push("collections/rixindex");
	if !store_item.exists() {
		cli::red_err(
			"The file: \"".to_owned()
				+ store_item.to_str().unwrap()
				+ "\" was not found !"
		);
		return std::process::ExitCode::FAILURE;
	}
	store_file_handle = io::BufReader::new(fs::File::open(&store_item)
		.unwrap()
	);
	bo_read_try = store_file_handle.read_u64::<BigEndian>();
	if bo_read_try.is_err() {
		cli::red_err(store_read_err(store_item));
		return std::process::ExitCode::FAILURE;
	}
	store_item_number = bo_read_try.unwrap();
	for i in 0..store_item_number {
		io_read_try = store_file_handle.read(
			&mut store_bin_content[0..12 as usize]
		);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..12 as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_id_str = store_str_parse.unwrap();

		bo_read8_try = store_file_handle.read_u8();
		if bo_read8_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_length = bo_read8_try.unwrap();

		if store_bin_content.len() < store_str_length as usize {
			store_bin_content.resize(store_str_length as usize, 0);
		}
		io_read_try = store_file_handle.read(
			&mut store_bin_content[0..store_str_length as usize]
		);
		if io_read_try.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		store_str_draft = store_bin_content[0..store_str_length as usize].to_vec();
		store_str_parse = String::from_utf8(store_str_draft);
		if store_str_parse.is_err() {
			cli::red_err(store_read_err(store_item));
			return std::process::ExitCode::FAILURE;
		}
		collections_list.insert(store_id_str, store_str_parse.unwrap());
	}

	for s in singleton_files {
		println!("{}", s.0);
		println!("{:?}", s.1);
	}
	println!("\n");
	for s in singletons {
		println!("{}", s.0);
		println!("{:?}", s.1);
	}
	println!("\n");
	for s in collections_list {
		println!("{} => {}", s.0, s.1);
	}

	return std::process::ExitCode::SUCCESS;
}