use std::env;

use toml::Parser;

pub struct Settings {
    // UI settings
    pub w: u32,
    pub h: u32,

    // Defaults
    pub default_project_paths: Vec<String>
}

const CONFIG_FILE_NAME: &'static str = ".foundryrc";

impl Settings {
    pub fn default() -> Settings {
        Settings {
            w: 800,
            h: 600,
        }
    }

    pub fn load() -> Settings {
        let home = env::home_dir() {
            Some(ref p) => Path::new(p),
            None => panic!("Could not get home directory"),
        };

        let file_path = home.join(CONFIG_FILE_NAME);

        let reader = match File::open(file_path) {
            Ok(_) => <,
            Err(why) => {

            },
        };

        let value = match Parser::new(toml).parse() {
            Some(table) => {

            },
            None => {

            },
        }

    }
}
