use clap::ArgMatches;

use crate::basics;

pub fn main(matches: &ArgMatches) -> std::process::ExitCode {
	println!("{:#?}", matches);
	return std::process::ExitCode::SUCCESS;
}