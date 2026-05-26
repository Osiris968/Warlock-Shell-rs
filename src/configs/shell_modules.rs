use std::collections::HashMap;

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
