use std::fs::File;

use crate::common::models::config::ConfigSettings;
use crate::constants::file_paths::FilePaths;

pub struct SettingFiles {
    pub config: ConfigSettings,
}

impl<'a> SettingFiles {
    fn config_file() -> ConfigSettings {
        let file_path = FilePaths::CONFIG;
        let f = File::open(file_path);
        let f_ok = match f {
            Ok(f) => f,
            Err(e) => {
                paniq!(
                    "An error occurred while reading the '{}' file (P00002): {:?}",
                    file_path,
                    e
                );
            }
        };

        let data: Result<ConfigSettings, serde_yaml::Error> = serde_yaml::from_reader(f_ok);

        match data {
            Ok(d) => d,
            Err(e) => {
                paniq!(
                    "An error occurred while deserializing the '{}' file (P00003): {:?}",
                    file_path,
                    e
                );
            }
        }
    }

    pub fn new() -> SettingFiles {
        log::debug!("reading the config files...");

        let c = SettingFiles::config_file();

        SettingFiles { config: c }
    }
}
