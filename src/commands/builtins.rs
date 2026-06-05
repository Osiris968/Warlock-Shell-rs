// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// builtins.rs

use std::env;
use std::io::{self, Write};
use std::path;

use crate::parse_commands;

pub fn clear_screen() -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

pub fn print_help() {
    println!("Codename Warlock Shell");
    println!("Copyright @ Michael Kestner");

    println!();

    println!("Supported Commands:");
    println!("exit");
    println!("help");
    println!("cd");
    println!("pipes");
}

pub fn change_directory(new_path: &path::Path) {
    if let Err(e) = env::set_current_dir(new_path) {
        eprintln!("Failed to change directory: {}", e);
    }
}

// Check what kind of command the user supplied. E.g. cd, ls, etc.
pub fn command_type(command: &str) {
    // NO infinite loop!
    if command == "type" {
        println!("{} is builtin", command);
        return;
    }

    let check_command: &str = &format!("/{command}");

    if parse_commands(&vec![command]) != 0 {
        println!("{} is a builtin", command);
        return;
    }

    let user_path = match std::env::var("PATH") {
        Ok(path) => path,
        Err(e) => {
            println!("Could not determine $PATH: {e}");
            return;
        }
    };

    let paths: Vec<&str> = user_path.split(':').collect();

    for dir in &paths {
        let path_str = String::from(*dir) + check_command;
        if path::Path::new(&path_str).exists() {
            println!("{} is {}", command, path_str);
            return;
        }
    }

    println!("type: Could not find '{}'", command);
}
