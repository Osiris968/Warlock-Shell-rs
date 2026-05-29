use std::collections::HashMap;
use std::io::{self, Write};

use shellrs::configs::{read_configs, shell_modules::parse_aliases};
use shellrs::{build_shell_prompt, fork_and_exec, parse_commands};

fn main() -> io::Result<()> {
    let config_map: HashMap<String, String> = read_configs()?;
    let prompt_color: &str = match config_map.get("prompt_color") {
        Some(val) => val,
        None => "green",
    };
    let alias_map = parse_aliases(&config_map)?;

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

        let code = parse_commands(&args);

        if code == 1 {
            continue;
        } else if code == 255 {
            break;
        }

        // O(n) for the alias map, but hopefully that should never be too large.
        if alias_map.contains_key(args[0]) {
            let arg_vec_with_alias: Vec<&str> =
                alias_map[args[0]].split_whitespace().collect();
            fork_and_exec(&arg_vec_with_alias)?;
            continue;
        }
        fork_and_exec(&args)?;
    }

    Ok(())
}
