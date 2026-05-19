use shellrs::configuration::configs::{self, read_configs};
use std::io;

fn main() -> io::Result<()> {
    if !configs::create_config_file() {
        return Err(io::Error::other("Could not create config file."));
    }

    read_configs()?;

    Ok(())
}
