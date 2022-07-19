use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("migrate")
        .about("Migrate a sql database")
        .version("v0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("dump")
                .short_flag('d')
                .long_flag("dump")
                .about("Dump sql scripts from database.")
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("compare")
                .short_flag('c')
                .long_flag("compare")
                .about("Compare sql scripts from database.")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("dump", sync_matches)) => {

        }
        Some(("compare", query_matches)) => {


        }
        _ => unreachable!(),
    }
}