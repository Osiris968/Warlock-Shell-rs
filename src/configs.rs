// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// configs.rs
#![allow(unused)]
use std::env;
use std::io;
use std::path;

use shellrs::get_home_directory;

pub fn get_config_path() -> io::Result<String> {
    let home_dir = get_home_directory()?;
    let config_path = "/.config/warlock-shell/warlock.conf";

    Ok(home_dir + config_path)
}

fn config_file_exists() -> bool {
    if let Ok(config_path) = get_config_path() {
        path::Path::new(&config_path).exists()
    } else {
        false
    }
}

pub fn create_config_file() {}
