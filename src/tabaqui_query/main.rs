use std::error;

use futures::prelude::*;

use super::cmd_get;
use super::cmd_ids;

#[derive(Debug, Fail)]
pub enum MainFailure {
    #[fail(
        display = "MainFailure::BaseUriNotProvided: Either the 'base-uri' CLI-argument should be passed or 'BASE_URI' env-var should be set"
    )]
    BaseUriNotProvided,

    #[fail(display = "MainFailure::CmdGetFailure: {}", _0)]
    CmdGetFailure(#[cause] cmd_get::CmdGetFailure),

    #[fail(display = "MainFailure::CmdIdsFailure: {}", _0)]
    CmdIdsFailure(#[cause] cmd_ids::CmdIdsFailure),
}

pub fn main<'a, 'b>(clap: clap::App<'a, 'b>) -> Result<(), MainFailure> {
    let matches = clap.get_matches();

    if matches.is_present("help") {
        print_usage(matches)?;

        Ok(())
    } else {
        let base_uri = config_base_uri(&matches)?;

        match matches.subcommand() {
            ("ids", Some(submatches)) => run_future(cmd_ids::run(&base_uri, &matches, submatches))
                .map_err(MainFailure::CmdIdsFailure),

            ("get", Some(submatches)) => run_future(cmd_get::run(&base_uri, &matches, submatches))
                .map_err(MainFailure::CmdGetFailure),

            (unknown_cmd, _) => handle_unknown_cmd(unknown_cmd),
        }
    }
}

fn run_future<F: Future<Item = I, Error = E>, I, E>(f: F) -> Result<I, E> {
    tokio::runtime::current_thread::Runtime::new()
        .expect("Failed to create Tokio-runtime")
        .block_on(f)
}

fn config_base_uri<'a>(matches: &clap::ArgMatches<'a>) -> Result<String, MainFailure> {
    if let Some(base_uri) = matches.value_of("base-uri") {
        Ok(base_uri.to_owned())
    } else if let Ok(base_uri) = std::env::var("BASE_URI") {
        Ok(base_uri)
    } else {
        Err(MainFailure::BaseUriNotProvided)
    }
}

fn handle_unknown_cmd(unknown_cmd: &str) -> Result<(), MainFailure> {
    Ok(println!("Unknown sub-command: {:?}", unknown_cmd))
}

fn print_usage<'a>(matches: clap::ArgMatches<'a>) -> Result<(), MainFailure> {
    println!("Usage:\n{}", matches.usage());
    Ok(())
}
