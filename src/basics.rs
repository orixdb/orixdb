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

#[allow(dead_code)]
pub fn white_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[37m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn black_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[30m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn blue_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[34m");
	println!("{}", s);
	println!("\x1b[0m");
}
#[allow(dead_code)]
pub fn red_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[31m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn green_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[32m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn yellow_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[33m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn cyan_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[36m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn magenta_out(s: String) {
	println!("\x1b[1m");
	println!("\x1b[35m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn white_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[37m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn black_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[30m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn blue_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[34m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn red_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[31m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn green_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[32m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn yellow_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[33m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn cyan_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[36m");
	println!("{}", s);
	println!("\x1b[0m");
}

#[allow(dead_code)]
pub fn magenta_err(s: String) {
	println!("\x1b[1m");
	println!("\x1b[35m");
	println!("{}", s);
	println!("\x1b[0m");
}