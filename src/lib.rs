#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate failure;

#[macro_use]
mod macros;

mod tabaqui_main;
pub use tabaqui_main::main as tabaqui_main;

pub mod data;
pub mod http_server;
pub mod storage;
