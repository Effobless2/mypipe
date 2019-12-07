extern crate clap;
use clap::App;

fn main() { 
    App::new("mypipe")
       .version("1.0")
       .about("Does great things!")
       .author("Maxime D.")
       .get_matches(); 
}
