use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, execvp, fork},
};
use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::path;

use shellrs::build_shell_prompt;
use shellrs::get_home_directory;
use shellrs::print_help;

use shellrs::configuration::configs;

// Invokes an appropriate syscall from the exec family.
fn my_exec(arg_list: &[&str]) {
    if arg_list.is_empty() {
        return;
    }

    let file_name_cstring: CString = CString::new(arg_list[0]).unwrap();
    let file_name: &CStr = file_name_cstring.as_c_str();

    let c_strings: Vec<CString> = arg_list.iter().map(|s| CString::new(*s).unwrap()).collect();

    let c_str_refs: Vec<&CStr> = c_strings.iter().map(|cs| cs.as_c_str()).collect();

    execvp(file_name, &c_str_refs).unwrap_err();

    println!("Command not found: {}", arg_list[0]);
}

// Invoke a fork syscall and, if I am the child process, call the myExec
// function that will execute the command passed in via argList
fn fork_and_exec(arg_list: &[&str]) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            my_exec(arg_list);
        }
        Err(_) => eprintln!("Fork failed!"),
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

fn parse_commands(arg_list: Vec<&str>) -> i32 {
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

    if let Some(first) = arg_list.first() {
        if *first == "exit" {
            return 255;
        } else if *first == "help" {
            print_help();
            return 1;
        } else if *first == "cd" {
            match arg_list.len() {
                1 => change_directory(path::Path::new(&home_dir)),
                _ => change_directory(path::Path::new(arg_list[1])),
            };
            return 1;
        }
    }
    0
}

fn main() -> io::Result<()> {
    loop {
        print!("{}", build_shell_prompt());

        if let Err(e) = io::stdout().flush() {
            eprintln!("Unable to flush stdout: {}", e);
        }

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.is_empty() {
            continue;
        }

        let args: Vec<&str> = user_input.split_whitespace().collect();

        let code = parse_commands(args.clone());

        if code == 1 {
            continue;
        } else if code == 255 {
            break;
        }

        fork_and_exec(&args);
    }

    Ok(())
}
