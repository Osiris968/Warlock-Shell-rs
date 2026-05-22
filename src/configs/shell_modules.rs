use crate::configs;
use std::collections::HashMap;
use std::io;

pub fn prompt_color() -> io::Result<String> {
    let color_map: HashMap<&str, &str> = HashMap::from([
        ("red", "\x1b[31m"),
        ("green", "\x1b[32m"),
        ("yellow", "\x1b[33m"),
        ("blue", "\x1b[34m"),
        ("purple", "\x1b[35m"),
        ("cyan", "\x1b[36m"),
        ("white", "\x1b[37m"),
    ]);

    let config_map: HashMap<String, String> = configs::read_configs()?;

    for (key, value) in config_map {
        if key == "prompt_color" && color_map.contains_key(value.as_str()) {
            return Ok(String::from(color_map[value.as_str()]));
        }
    }

    Ok(String::from(color_map["green"]))
}
