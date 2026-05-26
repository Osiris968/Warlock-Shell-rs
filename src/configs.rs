// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// configs.rs

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path;

use crate::get_home_directory;

pub mod shell_modules;

fn get_config_path() -> io::Result<String> {
    let home_dir = get_home_directory()?;
    let config_path = "/.config/warlock-shell/warlock.conf";

    Ok(home_dir + config_path)
}

fn config_file_exists() -> bool {
    if let Ok(config_path) = get_config_path() {
        path::Path::new(&config_path).exists()
    } else {
        eprintln!("Could not find path to config");
        false
    }
}

pub fn create_config_file() -> bool {
    // Config file already exists, nothing to be done.
    if config_file_exists() {
        eprintln!("Config file already exists. ");
        eprintln!("Remove existing one before trying to generate a new one.");
        return false;
    }

    // Find the path needed to create the file.
    let file_path: String = if let Ok(path) = get_config_path() {
        path
    } else {
        eprintln!("Could not find path to config");
        return false;
    };

    // Find the parent directory for the config file.
    // dirs::config_dir() returns a PathBuf, so we need to turn it into a string.
    let directory_path: String = if let Some(path) = dirs::config_dir() {
        // dirs::config_dir() returned Some(path).
        match path.to_str() {
            Some(path) => String::from(path) + "/warlock-shell",
            None => {
                eprintln!("Could not convert config path to string");
                return false;
            }
        }
        // dirs::config_dir() returned None.
    } else {
        eprintln!("Could not find config directory");
        return false;
    };

    if fs::create_dir_all(&directory_path).is_err() {
        eprintln!("Could not create config directory");
        return false;
    }

    if fs::File::create(&file_path).is_err() {
        eprintln!("Could not create config file");
        return false;
    }

    let data = "# Welcome to the Warlock Shell!
# This is where you can configure the shell's behaviors.
# A full list of supported configs are available on the wiki.
";

    if fs::write(&file_path, data).is_err() {
        eprintln!("Could not write data to new file");
        return false;
    }

    true
}

pub fn read_configs() -> io::Result<HashMap<String, String>> {
    if !config_file_exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No current config file exists. Create a new one with \"warlock_gen_config\" to continue.",
        ));
    }

    let file_path = get_config_path()?;
    let file = fs::File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut config_map: HashMap<String, String> = HashMap::new();

    // line is a Result<String, Error>.
    for line in reader.lines() {
        let line = line?;
        // Skips line if it's a comment or if it's empty.
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Split key value pairs by '=' and get rid of comments.
        let strs: Vec<&str> = line
            .split_once('#') // Split once returns a (before, after) tuple.
            .map(|(before, _)| before) // This map discards the after part of the tuple.
            .unwrap_or(&line) // If there is no '#', just use the whole string.
            .split('=') // Then split on the '='
            .collect();

        if strs.len() <= 1 {
            eprintln!("Either key or value is missing, skipping line.");
            continue;
        } else if strs.len() > 2 {
            eprintln!("Line has more than one equal sign, skipping line.");
            continue;
        }

        let strs: Vec<&str> = strs.iter().map(|&str| str.trim()).collect();

        if strs[0].is_empty() || strs[1].is_empty() {
            eprintln!("Either key or value is missing, skipping line.");
            continue;
        }

        config_map.insert(String::from(strs[0]), String::from(strs[1]));
    }

    Ok(config_map)
}
