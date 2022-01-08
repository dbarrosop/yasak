use base64;
use clap::{App, Arg, ArgMatches};

const HELP: &str = "You need to specify a subcommand. For more information try --help";

pub fn subcommand<'a>() -> App<'a> {
    return App::new("jwt")
        .about("Various utilities to deal with JWTs")
        .subcommands(vec![App::new("decode")
            .about("Decodes a JWT")
            .args(&[Arg::new("token").help("Token to decode")])]);
}

pub fn process(app_m: &ArgMatches) {
    match app_m.subcommand() {
        Some(("decode", sub_m)) => decode_token(sub_m),
        _ => {
            println!("{}", HELP)
        }
    }
}

fn decode_token(app_m: &ArgMatches) {
    let token: &str;
    match app_m.value_of("token") {
        Some(t) => token = t,
        None => {
            println!("you need to specify a token");
            return;
        }
    }
    let parts: Vec<&str> = token.split(".").collect();

    if parts.len() != 3 {
        println!("The specified token doesn't have exactly 3 parts so it must be wrong.")
    }

    let h = match base64::decode(parts[0]) {
        Ok(h) => h,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let header = std::str::from_utf8(&h).unwrap();

    let p = match base64::decode(parts[1]) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let payload = std::str::from_utf8(&p).unwrap();

    println!("{{\"header\":{},\"payload\":{}}}", &header, &payload);
}
