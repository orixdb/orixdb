use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Conf {
	pub name: String,
	pub display_name: String,
	pub description: String,
	pub full_description: String,
	pub version: String,
	pub author: String,
	pub full_author: String
}

pub fn get_conf() -> Conf {
	let cfg_str = include_str!("config.json");
	return serde_json::from_str(&cfg_str).unwrap();
}