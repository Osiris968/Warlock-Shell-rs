// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// fork_and_exec.rs

use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork},
};

use std::ffi::{CStr, CString};
use std::io;

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
