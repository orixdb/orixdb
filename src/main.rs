use std::collections::HashMap;

use clap::{ Command, Arg, ArgAction, ArgMatches };

mod basics;
mod create;
mod serve;
mod optimize;
mod upgrade;
mod copy;
mod convert;

fn main() -> std::process::ExitCode {
	let sub_commands: HashMap<
		&str, fn(&ArgMatches) -> std::process::ExitCode
	> = HashMap::from([
		("create", create::main as fn(&ArgMatches) -> std::process::ExitCode),
		("serve", serve::main as fn(&ArgMatches) -> std::process::ExitCode),
		("optimize", optimize::main as fn(&ArgMatches) -> std::process::ExitCode),
		("upgrade", upgrade::main as fn(&ArgMatches) -> std::process::ExitCode),
		("copy", copy::main as fn(&ArgMatches) -> std::process::ExitCode),
		("convert", convert::main as fn(&ArgMatches) -> std::process::ExitCode)
	]);

	let conf = basics::get_conf();
	let matches = Command::new(conf.display_name)
		.version(conf.version)
		.author(conf.author)
		.about(conf.description)
		.subcommand_required(true)

		.subcommand(Command::new("create")
			.about("To create a new OrixDB store.")
			.arg(
				Arg::new("folder")
					.required(false)
					.help("Folder to create for the new store.")
					.long_help("\
						Folder to create for the new store.\n\
						If this arg is not provided, then\n\
						the current directory is used.\
					")
			)
			.arg(
				Arg::new("name")
					.long("name")
					.short('n')
					.required(false)
					.help("The name of the new store.")
					.long_help("\
						The name of the new store.\n\
						If it's not set, it is defaulted\n\
						to the current directory's name\
					")
			)
			.arg(
				Arg::new("id")
					.long("id")
					.short('i')
					.required(false)
					.help("The id of the new store.")
					.long_help("\
						The id of the new store.\n\
						If it's not set, it is defaulted\n\
						to the current name's slug\
					")
			)
			.arg(
				Arg::new("type")
					.long("type")
					.short('t')
					.required(false)
					.help("The type of the new store.")
					.long_help("\
						The type of the new store.\n\
						Allowed values are: \"live\" (default),\n\
						\"lite\", \"backup\" and \"archive\".\
					")
			)
			.arg(
				Arg::new("ordering")
					.long("ordering")
					.short('o')
					.action(ArgAction::SetTrue)
					.required(false)
					.help("Whether or not the data is ordered during serving.")
					.long_help("\
						Whether or not live data ordering is active.\n\
						When this option is active, the data files are constantly\n\
						defragmented.
					")
			)
			.arg(
				Arg::new("checksumming")
					.long("checksum")
					.short('c')
					.action(ArgAction::SetTrue)
					.required(false)
					.help(
						"Whether or not checksums are used to ensure data integrity."
					)
					.long_help("\
						Whether or not checksums are used to ensure data integrity..\n\
						When this option is active, checksum files are constantly\n\
						generated to make sure that the data aren't corrupted.
					")
			)
			.arg(
				Arg::new("logging")
					.long("logging")
					.short('l')
					.required(false)
					.help("The logging type of the new store.")
					.long_help("\
						The logging type of the new store.\n\
						Allowed values are: \"off\", \"minimal\",\n\
						\"normal\" (default) and \"detailed\".\
					")
			)
		)

		.subcommand(Command::new("serve")
			.about("To launch a server for reading and updating a store.")
		)

		.subcommand(Command::new("optimize")
			.about("To optimize the data organization of a store.")
		)

		.subcommand(Command::new("upgrade")
			.about("To upgrade a store from a old version to a new one.")
		)

		.subcommand(Command::new("check")
			.about("To check if all the data in a store in correct (not corrupted).")
		)

		.subcommand(Command::new("archive")
			.about("To create an archive of a store.")
		)

		.subcommand(Command::new("restore")
			.about("To restore a store archive.")
		)

		.subcommand(Command::new("copy")
			.about("To create a duplicate of a store with different IDs.")
		)

		.subcommand(Command::new("convert")
			.about("To convert a store from one type to another")
		)

		.get_matches();

	let sub = matches.subcommand().unwrap();
	return sub_commands.get(sub.0).unwrap()(sub.1);
}
