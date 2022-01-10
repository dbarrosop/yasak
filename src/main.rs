mod jwt;
mod uuid;

use clap::App;
use std::error::Error;

const HELP: &str = "You need to specify a subcommand. For more information try --help";

fn main() -> Result<(), Box<dyn Error>> {
    let app_m = App::new("yasak - Yet Another Swiss Army Knife")
        .version("0.1.0")
        .author("David Barroso <@dbarrosop>")
        .about("Does everything you need... probably, maybe, perhaps...")
        .subcommands(vec![uuid::subcommand(), jwt::subcommand()])
        .get_matches();

    match app_m.subcommand() {
        Some(("uuid", sub_m)) => uuid::process(sub_m)?,
        Some(("jwt", sub_m)) => jwt::process(sub_m)?,
        _ => return Err(HELP.into()),
    };

    return Ok(());
}
