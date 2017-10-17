extern crate mpd;

use mpd::Client;

fn main() {
    let mut conn = match Client::connect("127.0.0.1:6600") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not connect to mpd");
            std::process::exit(1);
        }
    };

    let status = conn.status().unwrap();

    let (elapsed, total) = match status.time {
        Some(t) => t,
        None => std::process::exit(0),
    };

    print!(
        "{}:{:02}/{}:{:02} ({}%)",
        elapsed.num_minutes(),
        elapsed.num_seconds() - (elapsed.num_minutes() * 60),
        total.num_minutes(),
        total.num_seconds() - (total.num_minutes() * 60),
        (elapsed.num_seconds() as f64 / total.num_seconds() as f64 * 100.0).trunc()
    );
}
