// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// configs.rs

// #![allow(unused)]

use std::env;
use std::fs;
use std::io;
use std::path;

use shellrs::get_home_directory;

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
    if config_file_exists() {
        eprintln!("Config file already exists. ");
        eprintln!("Remove existing one before trying to generate a new one.");
        return false;
    }

    // Populate the path needed to create the file.
    let file_path: String = if let Ok(path) = get_config_path() {
        path
    } else {
        eprintln!("Could not find path to config");
        return false;
    };

    let directory_path: String = if let Some(path) = dirs::config_dir() {
        match path.to_str() {
            Some(path) => String::from(path) + "/warlock-shell",
            None => {
                eprintln!("Could not convert config path to string");
                String::from("ERROR")
            }
        }
    } else {
        eprintln!("Could not find config directory");
        return false;
    };

    if fs::create_dir(&directory_path).is_err() {
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

    // TODO: Remove unwrap.
    fs::write(&file_path, data).unwrap();

    true
}
