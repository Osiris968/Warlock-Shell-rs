// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// builtins.rs

use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork},
};
use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::path;

use crate::parse_commands;

pub fn clear_screen() -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

pub fn print_help() {
    println!("Warlock Shell");
    println!("Copyright @ Michael Kestner");

    println!();

    println!("Supported Builtin Commands:");
    println!("exit");
    println!("help");
    println!("cd");
    println!("clear");
    println!("type");
    println!("pipes");
    println!("chains (&&)");
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

// Invokes an appropriate syscall from the exec family.
fn my_exec(arg_list: &[&str]) -> io::Result<()> {
    if arg_list.is_empty() {
        return Ok(());
    }

    // Must be owned first before you can borrow it.
    let file_name_cstring: CString = CString::new(arg_list[0])?;
    let file_name: &CStr = file_name_cstring.as_c_str();

    let c_strings: Vec<CString> = arg_list
        .iter()
        .map(|s| CString::new(*s))
        // Ignore the values that error, if applicable.
        .filter_map(Result::ok)
        .collect();

    let c_str_refs: Vec<&CStr> =
        c_strings.iter().map(|cs| cs.as_c_str()).collect();

    // This doesn't crash the program, instead just continues.
    execvp(file_name, &c_str_refs).unwrap_err();

    println!("Command not found: {}", arg_list[0]);
    Ok(())
}

// Invoke a fork syscall and, if I am the child process, call the my_exec
// function that will execute the command passed in via arg_list.
pub fn fork_exec(arg_list: &[&str]) -> io::Result<()> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::other(e)),
        },
        Ok(ForkResult::Child) => my_exec(arg_list),
        Err(e) => Err(io::Error::other(e)),
    }
}
