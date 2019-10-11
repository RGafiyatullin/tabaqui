use std::str::FromStr;

use futures::prelude::*;
use hyper::http::uri::InvalidUri;
use hyper::http::Uri;

use super::http_get;
use super::HttpGetFailure;

#[derive(Debug, Fail)]
pub enum CmdIdsFailure {
    #[fail(display = "CmdIdsFailure::Unimplemented")]
    Unimplemented,

    #[fail(display = "CmdIdsFailure::HttpGetFailure: {}", _0)]
    HttpGetFailure(#[cause] HttpGetFailure),

    #[fail(display = "CmdIdsFailure::InvalidUri: {}", _0)]
    InvalidUri(#[cause] InvalidUri),
}

pub fn run<'a>(
    base_uri: &str,
    _matches: &clap::ArgMatches<'a>,
    _sub_matches: &clap::ArgMatches<'a>,
) -> impl Future<Item = (), Error = CmdIdsFailure> {
    let uri_fut = Uri::from_str(&format!("{}/__mgmt", base_uri))
        .map_err(CmdIdsFailure::InvalidUri)
        .into_future();
    let ids_fut =
        uri_fut.and_then(|uri| http_get::<Vec<u64>>(uri).map_err(CmdIdsFailure::HttpGetFailure));
    let done = ids_fut.map(pretty_print);
    done
}

fn pretty_print(ids: Vec<u64>) -> () {
    for id in ids {
        println!("{}", id)
    }
}
