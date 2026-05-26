#![allow(unused)]

use std::collections::HashMap;
use std::io;

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

// Take the output of one command and give it to another as arguments.
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

    println!("Left args: {:#?}", arg_list);
    println!("Right args: {:#?}", right_args);

    Ok(())
}

// TODO: Be able to handle command aliases.
