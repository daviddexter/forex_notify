use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use dirs::config_dir;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    major_currency: String,
    minor_currency: String,
    poll_frequency: u32,
}

fn main() {
    let config_name: &str = "forex_notify/config.yaml";

    println!("started forex_notify. Checking is config file exists");

    let config_path = {
        let o = config_dir().unwrap();
        let cnpn = o.as_path().join(config_name);
        cnpn
    };

    match config_path.exists() {
        true => listen(config_path),
        false => create_defualt_config(config_path),
    }
}

fn create_defualt_config(p: PathBuf) {
    // create the config then pass it to listen

    let config: Config = toml::from_str(
        r#"
        major_currency = 'USD'
        minor_currency = 'KES'
        poll_frequency = 2        
    "#,
    )
    .unwrap();

    assert_eq!(config.major_currency, "USD");
    assert_eq!(config.minor_currency, "KES");
    assert_eq!(config.poll_frequency, 2);

    listen(p);
}

fn listen(p: PathBuf) {
    println!("path is {:?}", p);
}
