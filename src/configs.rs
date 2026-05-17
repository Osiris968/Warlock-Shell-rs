// use std::env;
// use std::path;

use shellrs::get_home_directory;

pub fn _get_config_path() -> String {
    let _home_dir = get_home_directory();
    String::from("Hello")
}
