#[macro_use]
extern crate clap;
use clap::{App,};
use std::io::{self, Write};

fn main() { 
    let yaml = load_yaml!("../configs/main.yml");
    let matches = App::from_yaml(yaml).get_matches(); 

    let input = matches.value_of("in");
    let output = matches.value_of("out");

    let mut in_cmd : std::process::Command;
    let mut out_cmd : std::process::Command;
    let c_arg : &str;
    if cfg!(target_os = "windows"){
        in_cmd = std::process::Command::new("cmd");
        out_cmd = std::process::Command::new("cmd");
        c_arg = "/C";
    } else{
        in_cmd = std::process::Command::new("sh");
        out_cmd = std::process::Command::new("sh");
        c_arg = "-c";
    }
    let res_in = in_cmd
        .arg(c_arg)
        .arg(input.unwrap())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("error");
        
        let stdin = Some(res_in).map_or(
            std::process::Stdio::inherit(),
            |output: std::process::Child| std::process::Stdio::from(output.stdout.unwrap())
        );
    let out_res = out_cmd
        .arg(c_arg)
        .arg(output.unwrap())
        .stdin(std::process::Stdio::from(stdin))
        .output()
        .expect("error");
        
    io::stdout().write_all(&out_res.stdout).unwrap();
}

