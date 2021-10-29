use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use toml::Value;

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
    println!("############################################################")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub website: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub url: String,
}
