use std::env;
use std::io;
use std::path;

pub mod configuration {
    pub mod configs;
}

pub fn print_help() {
    println!("Codename Warlock Shell");
    println!("Copyright @ Michael Kestner");

    println!();

    println!("Supported Commands:");
    println!("exit");
    println!("help");
    println!("cd");
}

// Construct the shell's prompt from the username, hostname, and current path.
// Returns a formatted String with colors!
pub fn build_shell_prompt() -> String {
    let _red = "\x1b[31m";
    let green = "\x1b[32m";
    let reset = "\x1b[0m";

    let username = match whoami::username() {
        Ok(username) => username,
        Err(e) => {
            eprintln!("Unable to find username: {}", e);
            String::from("Error")
        }
    };
    let hostname = match whoami::hostname() {
        Ok(hostname) => hostname,
        Err(e) => {
            eprintln!("Unable to find device hostname: {}", e);
            String::from("Error")
        }
    };

    let path: String = if let Ok(current_path) = env::current_dir() {
        if let Ok(short_path) = condense_path(current_path) {
            short_path
        } else {
            eprintln!("Could not determine current directory");
            String::from("ERROR")
        }
    } else {
        eprintln!("Could not determine current directory");
        String::from("ERROR")
    };

    format!(
        "{}{}{}@{} {}{}{}> ",
        green, username, reset, hostname, green, path, reset
    )
}

// Take an absolute path and replace /home/{username} with ~.
// Returns a String instead of PathBuf for simplicity's sake.
pub fn condense_path(path: path::PathBuf) -> io::Result<String> {
    // Take the given path as a PathBuf and turn it into a String.
    let path_string = if let Some(s) = path.to_str() {
        String::from(s)
    } else {
        return Err(io::Error::other("Unable to convert PathBuf to String"));
    };

    // Find the home directory and turn it into a String.
    let home_dir = get_home_directory()?;

    // If the first part of the given path is identical to the home directory path, then replace it
    // with a '~' and return it.
    if path_string[0..home_dir.len()] == home_dir {
        return Ok(String::from('~') + &path_string[home_dir.len()..]);
    }

    Ok(path_string)
}

// Find the user's home directory.
pub fn get_home_directory() -> io::Result<String> {
    // Returns the home directory as a PathBuf.
    let home_path = match dirs::home_dir() {
        Some(home) => home,
        None => {
            return Err(io::Error::other("Could not find home directory"));
        }
    };

    // Change the PathBuf to a String.
    let home_string = match home_path.to_str() {
        Some(home) => home,
        None => return Err(io::Error::other("Could not convert home path to string")),
    };
    Ok(String::from(home_string))
}
