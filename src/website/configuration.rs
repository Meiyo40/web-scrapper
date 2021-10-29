use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use toml::Value;

pub fn set_configuration(filename: &str) {
    println!("seaching for: {}", &filename);
    let contents = fs::read_to_string(&filename).expect("Couldn't read configuration file");
    println!("{:?}", &contents);
    let data: Config = toml::from_str(&contents).unwrap();
    print_data(&data);
}

fn print_data(data: &Config) {
    println!("######################LOADED WEBSITES!######################");
    for website in &data.website {
        println!("{:?}", website);
    }
    println!("############################################################")
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    website: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Site {
    name: String,
    url: String,
}
