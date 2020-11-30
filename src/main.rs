use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::process;
use std::thread;
use std::time::Duration;
use std::{fs::create_dir, fs::File, io::Write, path::PathBuf};

extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};

use dirs::config_dir;

extern crate reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    major_currency: String,
    minor_currency: String,
    poll_frequency: u32,
}

fn main() {
    println!("started forex_notify");

    let config_name: &str = "forex_notify/config.yaml";

    thread::sleep(Duration::from_millis(1000));

    println!("retrieving config file");

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
    println!("cound not find config file. creating default one");

    let org = p.clone();
    // create the config then pass it to listen fn
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

    println!("{:?}", p);

    let dir = p.parent().unwrap();
    create_dir(dir).expect("failed to create config directory");

    let mut file = File::create(p.to_str().unwrap()).expect("error while creating config file");

    let tml = toml::to_string(&config).unwrap();

    file.write(tml.as_bytes())
        .expect("could not write to config toml file");

    listen(org);
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResp {
    code: String,
    rate: String,
}

fn listen(p: PathBuf) {
    println!("config file found at {:?}. setting up client", p);

    let mut file = File::open(p.to_str().unwrap()).expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read config file");

    let config: Config = toml::from_str(&contents).unwrap();

    if config.poll_frequency < 1 || config.poll_frequency > 23 {
        println!(
            "Invalid poll_frequency. Expected to range 1 - 23. Found {}",
            config.poll_frequency
        );
        process::exit(0);
    }

    let tick = format!("0 */{} * * * *", config.poll_frequency);

    let mut sched = JobScheduler::new();

    sched.add(Job::new(tick.parse().unwrap(), || {
        // let resp =
        //     reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
        // println!("{:#?}", resp);

        let resp = reqwest::blocking::get("https://httpbin.org/ip").expect("");
        println!("{:?}", resp);
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(100));
    }
}
