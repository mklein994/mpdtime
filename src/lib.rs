use mpd::{Client, State};
use std::env;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub struct Config {
    percent: bool,
    socket: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            percent: false,
            socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6600),
        }
    }
}

impl Config {
    pub fn from_args(mut args: env::Args) -> Result<Self> {
        args.next();

        let mut config = Config::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => eprintln!(
                    "Usage: {} [-p | --percent] \
                     [-c CONNECTION | --connection CONNECTION]",
                    env!("CARGO_PKG_NAME")
                ),
                "-p" | "--percent" => config.percent = true,
                "-c" | "--connection" => {
                    config.socket = args
                        .next()
                        .ok_or(Error::Arg("Connection flag needs an argument"))?
                        .parse()?
                }
                _ => {}
            }
        }

        if let Ok(host) = env::var("MPD_HOST") {
            config.socket.set_ip(host.parse()?);
        }

        if let Ok(port) = env::var("MPD_PORT") {
            config.socket.set_port(port.parse()?);
        }

        Ok(config)
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
    Arg(&'static str),
    Mpd(mpd::error::Error),
    Net(std::net::AddrParseError),
    ParseInt(std::num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Arg(err) => err.fmt(f),
            Error::Mpd(ref err) => err.fmt(f),
            Error::Net(ref err) => err.fmt(f),
            Error::ParseInt(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<mpd::error::Error> for Error {
    fn from(err: mpd::error::Error) -> Self {
        Error::Mpd(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Net(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn run(config: &Config) -> Result<()> {
    let mut conn = Client::connect(config.socket)?;

    let status = conn.status()?;

    if let Some((elapsed, total)) = status.time {
        if config.percent && !total.is_zero() {
            print!(
                "{}",
                elapsed.num_seconds() as f64 / total.num_seconds() as f64
            );
        } else {
            let state = match status.state {
                State::Play => PLAY_ICON,
                State::Pause => PAUSE_ICON,
                State::Stop => STOP_ICON,
            };

            let repeat = if status.repeat {
                if status.single {
                    REPEAT_ONE_ICON
                } else {
                    REPEAT_ICON
                }
            } else {
                ""
            };

            let shuffle = if status.random { SHUFFLE_ICON } else { "" };

            let percent =
                (elapsed.num_seconds() as f64 / total.num_seconds() as f64 * 100.0).trunc();

            let min = elapsed.num_minutes();
            let sec = elapsed.num_seconds() - (elapsed.num_minutes() * 60);
            let min_total = total.num_minutes();
            let sec_total = total.num_seconds() - (total.num_minutes() * 60);

            print!(
                "{state}{repeat}{shuffle} {min}:{sec:02}/{min_total}:{sec_total:02} ({percent}%)",
                state = state,
                repeat = repeat,
                shuffle = shuffle,
                min = min,
                sec = sec,
                min_total = min_total,
                sec_total = sec_total,
                percent = percent,
            );
        }
    }

    Ok(())
}
