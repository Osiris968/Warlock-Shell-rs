#![allow(unused)]

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;

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

// Take the output of one command and give it to the input of another.
pub fn handle_pipe(mut arg_list: Vec<&str>) -> io::Result<()> {
    // No pipe was provided.
    let index = match arg_list.iter().position(|x| x == &"|") {
        Some(val) => val,
        None => return Ok(()),
    };

    // split_off() splits the vector at an index, modifies the original, and returns a new vector
    // with the other elements.
    let mut right_args: Vec<&str> = arg_list.split_off(index);

    // Only given one command and the pipe. (Eg. ls -l |).
    if right_args.len() == 1 {
        return Err(io::Error::other("Nothing provided after pipe."));
    } else {
        right_args.remove(0);
    }

    let first_command = match arg_list.first() {
        Some(arg) => arg.to_string(),
        None => {
            eprintln!("Nothing provided before the pipe.");
            return Ok(());
        }
    };
    arg_list.remove(0);
    let second_command = match right_args.first() {
        Some(arg) => arg.to_string(),
        None => {
            eprintln!("Nothing provided after the pipe.");
            return Ok(());
        }
    };
    right_args.remove(0);

    let mut upstream = Command::new(first_command)
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

// TODO: Be able to handle command aliases.
fn parse_aliases(config_map: HashMap<String, String>) -> io::Result<()> {
    let mut alias_map: HashMap<String, String> = HashMap::new();
    for (key, value) in config_map {
        if key.contains("alias") {
            key.split_whitespace();
            alias_map.insert(value, key);
        }
    }
    Ok(())
}
