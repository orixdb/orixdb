use serde::{ Serialize, Deserialize };

use crate::cli;

#[derive(Deserialize)]
pub struct Conf {
	pub name: String,
	pub display_name: String,
	pub description: String,
	pub full_description: String,
	pub major: u16,
	pub minor: u16,
	pub patch: u16,
	pub author: String,
	pub full_author: String
}

#[derive(Serialize)]
pub struct Instance {
	pub verbosity: bool,
	pub api_port: u16,
	pub api_scan: bool,
	pub cluster_port: u16,
	pub cluster_scan: bool
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Serialize)]
pub enum LogLevel {
	Off,
	Minimal,
	Normal,
	Detailed
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Serialize)]
pub enum StoreType {
	Live,
	Lite,
	Backup,
	Archive
}

#[derive(Serialize)]
pub struct Store {
	pub name: String,
	pub id: String,
	pub version: String,
	pub kind: StoreType,
	pub ordering: bool,
	pub checksumming: bool,
	pub logging: LogLevel,
	pub defaults: Instance
}

pub fn get_conf() -> Conf {
	let cfg_str = include_str!("config.json");
	return serde_json::from_str(cfg_str).unwrap();
}

pub fn parse_port(port: &String, port_name: &str) -> (u16, bool) {
	let number_str;
	let ellipsis_str;
	let number;
	let test_0;
	let ellipsis;
	if port.find(".").is_some() {
		(number_str, ellipsis_str) = port.split_once(".").unwrap();
		if
			ellipsis_str.len() < 1 ||
			! ellipsis_str.chars().all(|c: char| c == '.')
		{
			cli::red_err(
				"The ".to_owned() + port_name
				+ " ellipsis must contain two or more periods.\n"
				+ "And nothing else. (Ex: 5500...)"
			);
			return (0, false);
		}
		ellipsis = true;
	}
	else {
		ellipsis = false;
		number_str = &*port;
	}
	number = number_str.parse::<u16>();
	test_0 = number.clone();
	if number.is_err() || test_0.unwrap() == 0 {
		cli::red_err(
			"The ".to_owned() + port_name
			+ " port must be a valid number between 1 and 65535, "
			+ "with an optional ellipsis at the end. (Ex: 5500...)"
		);
		return (0, false);
	}

	return (number.unwrap(), ellipsis);
}