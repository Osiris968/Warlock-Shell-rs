#![allow(unused)]

use shellrs::configuration::configs;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let config_map: HashMap<String, String> = configs::read_configs()?;

    for (key, value) in config_map {
        if key == "prompt_color" {
            todo!();
        }
    }

    Ok(())
}

struct ShellConfig {
    prompt_color: String,
    prompt_string_format: String,
}

impl ShellConfig {
    fn build(args: Vec<String>) -> io::Result<ShellConfig> {
        let mut args = args.iter();

        let prompt_color = match args.next() {
            Some(arg) => arg.to_owned(),
            None => {
                return Err(io::Error::other(
                    "Did not receive prompt color argument.",
                ));
            }
        };

        let prompt_string_format = match args.next() {
            Some(arg) => arg.to_owned(),
            None => {
                return Err(io::Error::other(
                    "Did not recieve prompt format argument",
                ));
            }
        };

        Ok(ShellConfig {
            prompt_color,
            prompt_string_format,
        })
    }
}
