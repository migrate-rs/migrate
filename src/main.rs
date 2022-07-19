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
                .about("Dump sql scripts from database")
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {}...", values);
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if *sync_matches
                .get_one::<bool>("info")
                .expect("defaulted by clap")
            {
                println!("Retrieving info for {}...", values);
            } else {
                println!("Installing {}...", values);
            }
        }
        Some(("query", query_matches)) => {


        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}