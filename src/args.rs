use docopt::Docopt;

const USAGE: &'static str = "
An IDE for Rust! Ugh!

Usage:
    foundry [options]
    foundry (--help | --version)

Options:
    -h --help         Show this help.
    -V --version      Show version info.
    -c --config=PATH  Optionally specify a config file to use.
";

#[derive(RustcDecodable)]
pub struct Args {
    // Optional flags
    pub flag_config: Option<String>,

    // Help and version info
    pub flag_help: bool,
    pub flag_version: bool,
}

pub fn get_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}
