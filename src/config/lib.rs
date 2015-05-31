extern crate rustc_serialize;

use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use rustc_serialize::json;

const CONFIG_DIR: &'static str = ".foundry";
const CONFIG_FILE_NAME: &'static str = "config.json";

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub session: Session,
    // Where new projects go
    pub project_dir: Option<String>,

}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Session {
    // Are we maximized?
    pub maximized: bool,
    // Dimensions for the window to goto when not fullscreen
    pub width: u32,
    pub height: u32,
    // What projects are currently open
    pub projects: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    HomeNotFound,
    Io(io::Error),
    Json(json::DecoderError),
}

impl Config {
    pub fn default() -> Config {
        Config {
            session: Session {
                maximized: true,
                width: 800,
                height: 600,
                projects: vec![],
            },
            project_dir: None,
        }
    }

    /// Load the IDE settings from the default location (~/.foundryrc)
    pub fn from_default_path() -> Result<Config, Error> {
        let home = match env::home_dir() {
            Some(p) => p,
            None => return Err(Error::HomeNotFound),
        };

        let mut file_path = home.join(CONFIG_DIR);
        file_path = file_path.join(CONFIG_FILE_NAME);

        Config::from_path(file_path)
    }

    /// Load the IDE settings from any location.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => return Err(Error::Io(why)),
        };

        let mut text = String::new();

        if let Err(why) = file.read_to_string(&mut text) {
            return Err(Error::Io(why));
        };

        match json::decode(&text) {
            Ok(conf) => Ok(conf),
            Err(why) => Err(Error::Json(why)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt :: Formatter) -> fmt::Result {
        match *self {
            Error::HomeNotFound => write!(f, "{}", "Could not find home directory ($HOME not set)."),
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Json(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HomeNotFound => "$HOME not set.",
            Error::Io(..) => "IO error while reading .foundry.json.",
            Error::Json(..) => "Failed to parse .foundry.json."
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            _ => None,
        }
    }
}
