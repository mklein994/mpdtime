extern crate mpd;

use mpd::Client;
use std::fmt;
use std::error;
use std::env;

#[derive(Debug, Default)]
pub struct Config {
    percent: bool,
}

impl Config {
    pub fn new() -> Self {
        let mut args = env::args();

        args.next();

        Self {
            percent: args.next()
                .and_then(|a| if a == "-p" { Some(a) } else { None })
                .is_some(),
        }
    }
}

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

pub fn run(config: &Config) -> Result<()> {
    let mut conn = Client::connect("127.0.0.1:6600")?;

    let status = conn.status()?;

    if let Some((elapsed, total)) = status.time {
        if config.percent && !total.is_zero() {
            print!(
                "{}",
                elapsed.num_seconds() as f64 / total.num_seconds() as f64
            );
        } else {
            print!(
                "{}:{:02}/{}:{:02} ({}%)",
                elapsed.num_minutes(),
                elapsed.num_seconds() - (elapsed.num_minutes() * 60),
                total.num_minutes(),
                total.num_seconds() - (total.num_minutes() * 60),
                (elapsed.num_seconds() as f64 / total.num_seconds() as f64 * 100.0).trunc()
            );
        }
    }

    Ok(())
}
