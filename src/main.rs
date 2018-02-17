extern crate mpdtime;

use mpdtime::Config;

fn main() {
    let config = Config::new();

    if let Err(e) = mpdtime::run(&config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
