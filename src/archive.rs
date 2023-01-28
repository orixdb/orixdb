use clap::ArgMatches;

pub fn main(matches: &ArgMatches) -> std::process::ExitCode {
	println!("{:#?}", matches);
	return std::process::ExitCode::SUCCESS;
}