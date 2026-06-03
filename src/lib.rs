use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork},
};

use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::path;

use crate::configs::shell_modules::prompt_color;
use crate::configs::shell_modules::{chain_commands, handle_pipe};

pub mod configs;

fn command_type(command: &str) -> io::Result<String> {
    // let path = std::env::var("PATH")?;
    let path = match std::env::var("PATH") {
        Ok(path) => path + command,
        Err(_) => return Err(io::Error::other("Could not determine $PATH")),
    };

    if parse_commands(&vec![command]) != 0 {
        return Ok(String::from("{} is builtin"));
    }

    Ok(String::from("no"))
}

// Invokes an appropriate syscall from the exec family.
fn my_exec(arg_list: &[&str]) -> io::Result<()> {
    if arg_list.is_empty() {
        return Ok(());
    }

    // Must be owned first before you can borrow it.
    let file_name_cstring: CString = CString::new(arg_list[0])?;
    let file_name: &CStr = file_name_cstring.as_c_str();

    // Ignore the values that error, if applicable.
    let c_strings: Vec<CString> = arg_list
        .iter()
        .map(|s| CString::new(*s))
        .filter_map(Result::ok)
        .collect();

    let c_str_refs: Vec<&CStr> = c_strings.iter().map(|cs| cs.as_c_str()).collect();

    // This doesn't crash the program, instead just continues.
    execvp(file_name, &c_str_refs).unwrap_err();

    println!("Command not found: {}", arg_list[0]);
    Ok(())
}

// Invoke a fork syscall and, if I am the child process, call the my_exec
// function that will execute the command passed in via arg_list.
pub fn fork_and_exec(arg_list: &[&str]) -> io::Result<()> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::other(e)),
        },
        Ok(ForkResult::Child) => my_exec(arg_list),
        Err(e) => Err(io::Error::other(e)),
    }
}

fn change_directory(new_path: &path::Path) {
    if let Err(e) = env::set_current_dir(new_path) {
        eprintln!("Failed to change directory: {}", e);
    }
}

fn expand_tilde(path: &str) -> String {
    let owned_path = String::from(path);
    if path.is_empty() {
        return owned_path;
    }

    let mut path_chars = path.char_indices();

    if path_chars.next() != Some((0, '~')) {
        return owned_path;
    }

    let home_dir = match get_home_directory() {
        Ok(home) => home,
        Err(_) => {
            eprintln!("Could not find home directory");
            return owned_path;
        }
    };

    if path.len() == 1 {
        return home_dir;
    }

    if path_chars.next() == Some((1, '/')) {
        return home_dir + &path[1..];
    }

    owned_path
}

pub fn parse_commands(arg_list: &Vec<&str>) -> i32 {
    // User supplied no arguments, we can just continue the loop.
    if arg_list.is_empty() {
        return 1;
    }

    // Translate ~ to the home directory.
    let expanded_args: Vec<String> = arg_list.iter().map(|arg| expand_tilde(arg)).collect();
    let arg_list: Vec<&str> = expanded_args.iter().map(|arg| arg.as_str()).collect();

    let home_dir = match get_home_directory() {
        Ok(home) => home,
        Err(_) => {
            eprintln!("Could not find home directory");
            return 1;
        }
    };

    // User wants to pipe a command into another.
    if arg_list.contains(&"|") {
        if let Err(e) = handle_pipe(arg_list) {
            eprintln!("{}", e);
        }
        return 0;
    }
    if arg_list.contains(&"&&") {
        if let Err(e) = chain_commands(arg_list) {
            eprintln!("{}", e);
        }
        return 0;
    }

    // Custom parsing for builtin commands.
    // These get executed before looking for aliases.
    if let Some(first) = arg_list.first() {
        match *first {
            "exit" => {
                return 255;
            }
            "help" => {
                print_help();
                return 1;
            }
            "cd" => {
                match arg_list.len() {
                    1 => change_directory(path::Path::new(&home_dir)),
                    _ => change_directory(path::Path::new(arg_list[1])),
                };
                return 1;
            }
            "warlock_gen_config" => {
                configs::create_config_file();
                return 1;
            }
            "clear" => match clear_screen() {
                Ok(_) => {
                    return 1;
                }
                Err(e) => {
                    eprintln!("{e}");
                    return 1;
                }
            },
            _ => {
                return 0;
            }
        }
    }
    0
}

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

// Construct the shell's prompt from the username, hostname, and current path.
// Returns a formatted String with colors!
pub fn build_shell_prompt(color: &str) -> String {
    let color = prompt_color(Some(color));
    let reset = "\x1b[0m";

    let username = match whoami::username() {
        Ok(username) => username,
        Err(e) => {
            eprintln!("Unable to find username: {}", e);
            String::from("ERROR")
        }
    };
    let hostname = match whoami::hostname() {
        Ok(hostname) => hostname,
        Err(e) => {
            eprintln!("Unable to find device hostname: {}", e);
            String::from("ERROR")
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
        color, username, reset, hostname, color, path, reset
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
        None => {
            return Err(io::Error::other("Could not convert home path to string"));
        }
    };
    Ok(String::from(home_string))
}
