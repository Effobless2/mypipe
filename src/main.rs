#[macro_use]
extern crate clap;
use clap::{App};

fn main() { 
    let yaml = load_yaml!("../configs/main.yml");
    let matches = App::from_yaml(yaml).get_matches(); 
}

