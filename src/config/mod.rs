use std::fs;
use std::io::Write;
use std::path::Path;

use dirs;

pub struct Config {
    config_dir: String,
    pub profile: String,
}

impl Config {
    pub fn new() -> Config {
        let c = Config {
            config_dir: format!(
                "{}/.vide",
                dirs::home_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            ),
            profile: "default".to_string(),
        };

        if !c.config_directory_exists() {
            c.create_config_directory();
            c.create_default_profile();
        }

        c
    }

    pub fn get_profile(&self) -> String {
        format!("{}/profiles/default/.vimrc", &self.config_dir)
    }

    fn config_directory_exists(&self) -> bool {
        Path::new(&self.config_dir).is_dir()
    }

    fn create_config_directory(&self) {
        fs::create_dir_all(format!("{}/profiles", &self.config_dir)).unwrap();
    }

    fn create_default_profile(&self) {
        fs::create_dir_all(format!("{}/profiles/default", &self.config_dir)).unwrap();
        let mut default_profile_file =
            fs::File::create(format!("{}/profiles/default/.vimrc", &self.config_dir)).unwrap();
        default_profile_file.write_all(b"set nu").unwrap();
    }
}
