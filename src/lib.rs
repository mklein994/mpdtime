extern crate mpd;

use mpd::Client;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    Mpd(mpd::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Mpd(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Mpd(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Mpd(ref err) => Some(err),
        }
    }
}

impl From<mpd::error::Error> for Error {
    fn from(err: mpd::error::Error) -> Self {
        Error::Mpd(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    let mut conn = match Client::connect("127.0.0.1:6600") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not connect to mpd");
            std::process::exit(1);
        }
    };

    let status = conn.status()?;

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

    Ok(())
}
