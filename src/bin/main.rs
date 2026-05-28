use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path;

use shellrs::build_shell_prompt;
use shellrs::configs::shell_modules::{handle_pipe, parse_aliases};
use shellrs::fork_and_exec;
use shellrs::get_home_directory;
use shellrs::print_help;

use shellrs::configs;

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

fn parse_commands(arg_list: &Vec<&str>) -> i32 {
    // User supplied no arguments, we can just continue the loop.
    if arg_list.is_empty() {
        return 1;
    }

    // Translate ~ to the home directory.
    let expanded_args: Vec<String> =
        arg_list.iter().map(|arg| expand_tilde(arg)).collect();
    let arg_list: Vec<&str> =
        expanded_args.iter().map(|arg| arg.as_str()).collect();

    let home_dir = match get_home_directory() {
        Ok(home) => home,
        Err(_) => {
            eprintln!("Could not find home directory");
            return 1;
        }
    };

    if arg_list.contains(&"|") {
        handle_pipe(arg_list.clone()).unwrap();
        return 1;
    }

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
        } else if *first == "warlock_gen_config" {
            configs::create_config_file();
            return 1;
        }
    }
    0
}

fn main() -> io::Result<()> {
    let config_map: HashMap<String, String> = configs::read_configs()?;
    let prompt_color: &str = match config_map.get("prompt_color") {
        Some(val) => val,
        None => "green",
    };
    let _alias_map = parse_aliases(&config_map);

    loop {
        print!("{}", build_shell_prompt(prompt_color));

        if let Err(e) = io::stdout().flush() {
            eprintln!("Unable to flush stdout: {}", e);
        }

        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            // Ctrl+D was given, break loop.
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        if user_input.is_empty() {
            continue;
        }

        let args: Vec<&str> = user_input.split_whitespace().collect();

        // TODO: Give alias_map to parse_commands to check against every time?
        let code = parse_commands(&args);

        if code == 1 {
            continue;
        } else if code == 255 {
            break;
        }

        fork_and_exec(&args)?;
    }

    Ok(())
}
