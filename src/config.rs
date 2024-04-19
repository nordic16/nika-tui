use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::constants::DEFAULT_CONFIG_DIR;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    anilist_token: String,
    chapter_page_size: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            anilist_token: Default::default(),
            chapter_page_size: 25,
        }
    }
}

impl Config {
    pub fn get_or_default() -> Self {
        let home_dir = env::var_os("HOME");

        if let Some(dir) = home_dir {
            let s = dir.into_string().unwrap();
            let fpath = format!("{s}/{DEFAULT_CONFIG_DIR}/config.toml");

            Self::ensure_conditions();

            match fs::read_to_string(&fpath) {
                // Note: there might still be errors in the config, so unwrap_or_default() is necessary.
                Ok(data) => toml::from_str(data.as_str()).unwrap_or_default(),
                Err(_) => {
                    fs::write(fpath, "").unwrap();
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    pub fn chapter_page_size(&self) -> u16 {
        self.chapter_page_size
    }

    pub fn anilist_token(&self) -> &str {
        &self.anilist_token
    }

    /// If dir doesn't exist, create it. As for the config file itself, it's handled on get_or_default(), so no need to
    /// handle the scenario where it doesn't exist here.
    fn ensure_conditions() {
        // Whether or not this command succeeds or not is irrelevant.
        let home_dir = env::var_os("HOME");

        if let Some(dir) = home_dir {
            let s = dir.into_string().unwrap();
            let path = format!("{s}/{DEFAULT_CONFIG_DIR}");
            let _ = fs::create_dir(&path);
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        let home_dir = env::var_os("HOME").unwrap().into_string().unwrap();
        let fpath = format!("{home_dir}/{DEFAULT_CONFIG_DIR}/config.toml");

        let s = toml::to_string(&self).unwrap();
        std::fs::write(fpath, s).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::{env, fs};

    #[test]
    fn load_existing_or_new() {
        let fname = "config.toml";

        let config = Config {
            anilist_token: "lkjasdjklasjlkdasjlk".into(),
            chapter_page_size: 25,
        };

        let home_dir = env::var_os("HOME");

        if let Some(dir) = home_dir {
            let path = format!("{}/.config/nika-tui/", dir.into_string().unwrap());
            let stuff = toml::to_string(&config).unwrap();

            // Whether or not this command succeeds or not is irrelevant.
            let _ = fs::create_dir(&path);

            let full_path = format!("{path}{fname}");

            match fs::write(full_path, &stuff) {
                Ok(_) => println!("Done!"),
                Err(_) => panic!("Couldn't write to file."),
            };
        } else {
            panic!("What? Couldn't find home dir.")
        }
    }

    #[test]
    fn test_get_or_default() {
        let s = Config::get_or_default();

        println!("{:?}", s);
    }
}
