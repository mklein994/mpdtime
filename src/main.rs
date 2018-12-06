use mpdtime::Config;

fn main() {
    let args = std::env::args();

    if let Err(e) = Config::from_args(args).and_then(|config| mpdtime::run(&config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
