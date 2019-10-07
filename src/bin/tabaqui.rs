#[macro_use]
extern crate clap;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv::dotenv().ok();

    let clap_spec_yml = load_yaml!("tabaqui.clap.yml");
    let clap = clap::App::from_yaml(clap_spec_yml);
    tabaqui::tabaqui_main(clap)
}
