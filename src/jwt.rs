use base64::decode;
use clap::{App, Arg, ArgMatches};
use std::error::Error;

const HELP: &str = "You need to specify a subcommand. For more information try --help";

pub fn subcommand<'a>() -> App<'a> {
    return App::new("jwt")
        .about("Various utilities to deal with JWTs")
        .subcommands(vec![App::new("decode")
            .about("Decodes a JWT")
            .args(&[Arg::new("token").help("Token to decode").required(true)])]);
}

pub fn process(app_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match app_m.subcommand() {
        Some(("decode", sub_m)) => Ok(decode_token(sub_m)?),
        _ => return Err(HELP.into()),
    }
}

fn decode_token(app_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let token = app_m.value_of("token").unwrap();
    let parts: Vec<&str> = token.split(".").collect();

    if parts.len() != 3 {
        println!("The specified token doesn't have exactly 3 parts so it must be wrong.")
    }

    let h = &decode(parts[0])?;
    let header = std::str::from_utf8(h)?;

    let p = &decode(parts[1])?;
    let payload = std::str::from_utf8(p)?;

    println!("{{\"header\":{},\"payload\":{}}}", &header, &payload);

    return Ok(());
}
