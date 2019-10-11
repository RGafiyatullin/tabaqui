use std::str::FromStr;

use futures::prelude::*;

use hyper::http::uri::InvalidUri;
use hyper::http::Uri;

use json_utils::json::JsValue;

use crate::data::Rq;
use crate::data::Rs;

use super::http_get;
use super::HttpGetFailure;

#[derive(Debug, Fail)]
pub enum CmdGetFailure {
    #[fail(display = "CmdGetFailure::Unimplemented")]
    Unimplemented,

    #[fail(display = "CmdGetFailure::HttpGetFailure: {}", _0)]
    HttpGetFailure(#[cause] HttpGetFailure),

    #[fail(display = "CmdGetFailure::InvalidUri: {}", _0)]
    InvalidUri(#[cause] InvalidUri),

    #[fail(display = "CmdGetFailure::InvalidId: {}", _0)]
    InvalidId(#[cause] std::num::ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Info {
    id: u64,
    rq: Option<Rq>,
    rs: Option<Rs>,
}

pub fn run<'a>(
    base_uri: &str,
    _matches: &clap::ArgMatches<'a>,
    sub_matches: &clap::ArgMatches<'a>,
) -> impl Future<Item = (), Error = CmdGetFailure> {
    let base_uri = base_uri.to_owned();
    let id_fut = sub_matches
        .value_of("ID")
        .expect("Mandatory argument 'ID' is missing")
        .parse::<u64>()
        .map_err(CmdGetFailure::InvalidId)
        .into_future();
    let uri_fut = id_fut.and_then(move |id| {
        Uri::from_str(&format!("{}/__mgmt/{}", base_uri, id)).map_err(CmdGetFailure::InvalidUri)
    });
    let object_fut =
        uri_fut.and_then(|uri| http_get::<Info>(uri).map_err(CmdGetFailure::HttpGetFailure));
    let done = object_fut.map(pretty_print_info);
    done
}

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

fn fmt_systemtime(t: SystemTime) -> String {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    format!("{}", datetime.format("%Y-%m-%d %T"))
}

fn pretty_print_info(info: Info) -> () {
    println!("ID: {}", info.id);
    pretty_print_rq_opt(info.rq);
    pretty_print_rs_opt(info.rs);
}

fn pretty_print_rq_opt(rq_opt: Option<Rq>) {
    if let Some(rq) = rq_opt {
        println!("RQ: at {}", fmt_systemtime(rq.at()));
        let rq_query = if let Some(q) = rq.query() {
            format!("?{}", q)
        } else {
            "".to_owned()
        };
        println!("\t{} {}{} HTTP/1.1", rq.method(), rq.path(), rq_query);

        for (h_key, h_value) in rq.headers() {
            println!("\t{}: {}", h_key, h_value);
        }
        println!("",);

        if let Ok(str_body) = std::str::from_utf8(rq.body()) {
            println!("{}", str_body);
        } else {
            println!("[BINARY]")
        }
    } else {
        println!("RQ: <undefined>")
    }
}

fn pretty_print_rs_opt(rs_opt: Option<Rs>) {
    if let Some(rs) = rs_opt {
        println!("RS: at {}", fmt_systemtime(rs.at()));

        let status_str = hyper::http::StatusCode::from_u16(rs.status())
            .map(|status| {
                format!(
                    "{} {}",
                    status.as_u16(),
                    status.canonical_reason().unwrap_or("UNKNOWN")
                )
            })
            .unwrap_or(format!("{} UNKNOWN", rs.status()));

        println!("\tHTTP/1.1 {}", status_str);

        for (h_key, h_value) in rs.headers() {
            println!("\t{}: {}", h_key, h_value);
        }
        println!("");

        if let Ok(str_body) = std::str::from_utf8(rs.body()) {
            println!("{}", str_body);
        } else {
            println!("[BINARY]")
        }
    } else {
        println!("RS: <undefined>")
    }
}
