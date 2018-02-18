extern crate mpd;

use mpd::{Client, State};
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

/// \u{e034} ⏸
pub const PAUSE_ICON: &str = "<span font_desc='Material Icons'>\u{e034}</span>";

/// \u{e037} ⏵
pub const PLAY_ICON: &str = "<span font_desc='Material Icons'>\u{e037}</span>";

/// \u{e047} ⏹
pub const STOP_ICON: &str = "<span font_desc='Material Icons'>\u{e047}</span>";

pub const SHUFFLE_ICON: &str = "<span font_desc='Material Icons'>\u{e043}</span>";

pub const REPEAT_ICON: &str = "<span font_desc='Material Icons'>\u{e040}</span>";

pub const REPEAT_ONE_ICON: &str = "<span font_desc='Material Icons'>\u{e041}</span>";

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
                "{}{}{} {}:{:02}/{}:{:02} ({}%)",
                match status.state {
                    State::Play => PLAY_ICON,
                    State::Pause => PAUSE_ICON,
                    State::Stop => STOP_ICON,
                },
                if status.repeat {
                    if status.single {
                        REPEAT_ONE_ICON
                    } else {
                        REPEAT_ICON
                    }
                } else {
                    ""
                },
                if status.random { SHUFFLE_ICON } else { "" },
                elapsed.num_minutes(),
                elapsed.num_seconds() - (elapsed.num_minutes() * 60),
                total.num_minutes(),
                total.num_seconds() - (total.num_minutes() * 60),
                (elapsed.num_seconds() as f64 / total.num_seconds() as f64 * 100.0).trunc(),
            );
        }
    }

    Ok(())
}
