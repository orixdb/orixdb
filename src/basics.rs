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

pub fn get_conf() -> Conf {
	let cfg_str = include_str!("config.json");
	return serde_json::from_str(&cfg_str).unwrap();
}
