use serde::{Deserialize, Serialize};
//use std::env;
use std::fs;
use std::time::{Duration, SystemTime};
//use toml::Value;

///return the list of website to process
pub fn set_configuration(filename: &str) -> Config {
    println!("Getting configuration in: {}", &filename);
    let contents = fs::read_to_string(&filename).expect("Couldn't read configuration file");
    let data: Config = toml::from_str(&contents).unwrap();
    print_data(&data);
    data
}

fn print_data(data: &Config) {
    println!("######################LOADED WEBSITES!######################");
    for website in &data.website {
        println!("{:?}", website);
    }
    println!("###################### LOADED USERS ! ######################");
    for user in &data.user_to_search {
        println!("{:?}", user);
    }
    println!("############################################################")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub website: Vec<Site>,
    pub user_to_search: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub last_update: u64,
}

impl User {
    ///Update the last_update prop as the system timestamp of now() in seconds.
    pub fn set_last_update(&mut self) -> &User {
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("Couldn't read timestamp"),
        };
        self.last_update = now;
        self
    }
}
