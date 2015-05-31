use std::error::Error as StdError;
use std::fmt;
use std::fs::PathExt;
use std::fs::walk_dir;
use std::fs::WalkDir;
use std::io;
use std::path::Path;

pub struct Project {
    files : WalkDir,
}

#[derive(Debug)]
pub enum Error {
    NoCargoFile,
    Io(io::Error),
}

impl Project {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project, Error> {
        let cargo_path = Path::join(path.as_ref(), "Cargo.toml");
        if !cargo_path.exists() {
            return Err(Error::NoCargoFile)
        }

        let files = walk_dir(path).map_err(|err| Error::Io(err));
        return files.map(|file_walk| Project { files : file_walk });
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt :: Formatter) -> fmt::Result {
        match *self {
            Error::NoCargoFile => write!(f, "{}", "There is no cargo file at the given folder path."),
            Error::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoCargoFile => "The given folder path does not exist or has no cargo file.",
            Error::Io(..) => "There was an IO error while constructing the project from the given folder.",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }
}
