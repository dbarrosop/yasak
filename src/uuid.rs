use clap::{App, Arg, ArgMatches};
use std::error::Error;
use uuid::Uuid;

const HELP: &str = "You need to specify a subcommand. For more information try --help";

pub fn subcommand<'a>() -> App<'a> {
    return App::new("uuid")
        .about("Various utilities to deal with uuids")
        .subcommands(vec![App::new("generate")
            .about("Generate new uuid in hyphenated format")
            .args(&[
                Arg::new("urn")
                    .short('u')
                    .long("urn")
                    .help("Output in urn format"),
                Arg::new("simple")
                    .short('s')
                    .long("simple")
                    .help("Output in simple format"),
            ])]);
}

pub fn process(app_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match app_m.subcommand() {
        Some(("generate", sub_m)) => {
            let uuid = Uuid::new_v4();

            if sub_m.is_present("urn") {
                println!("{}", uuid.to_urn());
            } else if sub_m.is_present("simple") {
                println!("{}", uuid.to_simple());
            } else {
                println!("{}", uuid);
            }
            return Ok(());
        }
        _ => return Err(HELP.into()),
    }
}

