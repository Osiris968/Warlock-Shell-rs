use shellrs::configuration::configs::read_configs;
// use shellrs::configuration::configs::{self, read_configs};
use std::io;

fn main() -> io::Result<()> {
    // configs::create_config_file();

    read_configs()?;

    Ok(())
}
