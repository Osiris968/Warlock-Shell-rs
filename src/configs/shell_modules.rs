// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// shell_modules.rs

use std::collections::HashMap;
use std::io;
use std::process::{Command, Stdio};

use crate::parse_commands;

pub fn prompt_color(c: Option<&str>) -> String {
    let color_map: HashMap<&str, &str> = HashMap::from([
        ("red", "\x1b[31m"),
        ("green", "\x1b[32m"),
        ("yellow", "\x1b[33m"),
        ("blue", "\x1b[34m"),
        ("purple", "\x1b[35m"),
        ("cyan", "\x1b[36m"),
        ("white", "\x1b[37m"),
    ]);

    let color = c.unwrap_or("green");

    match color_map.get(&color) {
        Some(val) => val.to_string(),
        None => String::from("\x1b[32m"),
    }
}

pub fn chain_commands(mut arg_list: Vec<&str>) -> io::Result<()> {
    let index = match arg_list.iter().position(|x| *x == "&&") {
        Some(val) => val,
        None => return Ok(()),
    };

    let mut right_args: Vec<&str> = arg_list.split_off(index);

    // Only given one command and the chain. (Eg. ls -l &&).
    if right_args.len() == 1 {
        return Err(io::Error::other("Nothing provided after &&."));
    } else {
        right_args.remove(0);
    }

    // Parse built in commands first.
    let builtin_first = parse_commands(&arg_list);
    let builtin_second = parse_commands(&right_args);

    let first_command = match arg_list.first() {
        Some(arg) => arg.to_string(),
        None => {
            return Err(io::Error::other("Nothing provided before &&."));
        }
    };
    arg_list.remove(0);

    let second_command = match right_args.first() {
        Some(arg) => arg.to_string(),
        None => {
            return Err(io::Error::other("Nothing provided after &&."));
        }
    };
    right_args.remove(0);

    // parse_commands returns 0 when the command given is not builtin. We do not want to execute a
    // binary if there is a builtin command of the same name.
    if builtin_first == 0 {
        // If this errors, the command never started.
        let command_status =
            Command::new(first_command).args(&arg_list).output()?;

        // If this errors, the command started but didn't finish.
        if !command_status.status.success() {
            return Err(io::Error::other(
                "First command failed, skipping second.",
            ));
        }
        // In either case, we do not want to start the second command.
    }

    if builtin_second == 0 {
        Command::new(second_command).args(&right_args).status()?;
    }

    Ok(())
}

// Take the output of one command and give it to the input of another.
pub fn handle_pipe(mut arg_list: Vec<&str>) -> io::Result<()> {
    // No pipe was provided.
    let index = match arg_list.iter().position(|x| *x == "|") {
        Some(val) => val,
        None => return Ok(()),
    };

    // split_off() splits the vector at an index, modifies the original, and returns a new vector
    // with the other elements.
    let mut right_args: Vec<&str> = arg_list.split_off(index);

    // Parse built in commands first.
    if parse_commands(&arg_list) != 0 || parse_commands(&right_args) != 0 {
        return Err(io::Error::other("Could not parse command."));
    }

    // Only given one command and the pipe. (Eg. ls -l |).
    if right_args.len() == 1 {
        return Err(io::Error::other("Nothing provided after pipe."));
    } else {
        right_args.remove(0);
    }

    let first_command = match arg_list.first() {
        Some(arg) => arg.to_string(),
        None => {
            return Err(io::Error::other("Nothing provided before the pipe"));
        }
    };
    arg_list.remove(0);
    let second_command = match right_args.first() {
        Some(arg) => arg.to_string(),
        None => {
            return Err(io::Error::other("Nothing provided after the pipe."));
        }
    };
    right_args.remove(0);

    let upstream = Command::new(first_command)
        .args(arg_list)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap();
    let downstream = Command::new(second_command)
        .args(right_args)
        .stdin(upstream)
        .spawn()?;

    let output = downstream.wait_with_output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

pub fn parse_aliases(
    config_map: &HashMap<String, String>,
) -> io::Result<HashMap<String, String>> {
    let mut alias_map: HashMap<String, String> = HashMap::new();

    for (key, value) in config_map {
        if key.contains("alias") {
            let tuple = match key.split_once(' ') {
                Some(t) => t,
                None => {
                    eprintln!("No whitespace found. Skipping line.");
                    continue;
                }
            };
            alias_map.insert(String::from(tuple.1), String::from(value));
        }
    }

    Ok(alias_map)
}
