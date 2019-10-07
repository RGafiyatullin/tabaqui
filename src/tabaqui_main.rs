use std::error;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use futures::prelude::*;

use hyper::Uri;
use log::LevelFilter as LogLevelFilter;

const DEFAULT_STORAGE_SIZE: usize = 100;

pub fn main<'a, 'b>(clap: clap::App<'a, 'b>) -> Result<(), Box<dyn error::Error>> {
    let matches = clap.get_matches();

    if matches.is_present("help") {
        print_usage(matches)?;

        Ok(())
    } else {
        set_log_level(&matches)?;
        let bind_addr: IpAddr = matches.value_of("bind-addr").unwrap_or("0.0.0.0").parse()?;
        let bind_port: u16 = matches.value_of("bind-port").unwrap_or("8080").parse()?;
        let bind_sock: SocketAddr = SocketAddr::from((bind_addr, bind_port));

        let storage_size: usize = matches
            .value_of("storage-size")
            .map(|s| s.parse())
            .unwrap_or(Ok(DEFAULT_STORAGE_SIZE))?;

        let backend_base_uri: Uri = Uri::from_str(
            matches
                .value_of("backend")
                .expect("Required Clap argument missing: backend"),
        )?;

        // let mgmt_path = matches
        //     .value_of("mgmt-path")
        //     .expect("Required Clap argument missing: mgmt-path");

        let (storage_api, storage_running) = crate::storage::create(storage_size);

        let routes = crate::http_server::routes(Arc::new(storage_api), backend_base_uri);
        let http_server_running = warp::serve(routes).bind(bind_sock);

        let app_running = http_server_running
            .map_err(|_| format_err!("HTTP-server failure").into())
            .join(storage_running.map_err(failure::Error::from))
            .map(|_| ());

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let () = runtime.block_on_all(app_running)?;
        Ok(())
    }
}

fn set_log_level<'a>(matches: &clap::ArgMatches<'a>) -> Result<(), Box<dyn error::Error>> {
    let log_level_filter: LogLevelFilter = match matches.occurrences_of("verbose") {
        0 => LogLevelFilter::Warn,
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        3 | _ => LogLevelFilter::Trace,
    };

    let () = env_logger::builder()
        .filter_level(log_level_filter)
        .try_init()?;

    Ok(())
}

fn print_usage<'a>(matches: clap::ArgMatches<'a>) -> Result<(), Box<dyn error::Error>> {
    println!("Usage:\n{}", matches.usage());
    Ok(())
}
