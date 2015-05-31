extern crate config;
extern crate docopt;
extern crate rustc_serialize;

use config::Config;
use foundry::Foundry;

mod args;
mod foundry;

fn main() {
    let args = args::get_args();

    let config = match args.flag_config {
        Some(s) => Config::from_path(s).unwrap(),
        None => Config::from_default_path().unwrap(),
    };


    println!("{:?}", config);
    let mut data = Foundry::new();
}
