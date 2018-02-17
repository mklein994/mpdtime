extern crate mpdtime;

fn main() {
    if let Err(e) = mpdtime::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
