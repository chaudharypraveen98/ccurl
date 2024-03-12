extern crate clap;

use clap::{ Arg, Command};

fn main() {
    println!("Hello, world!");
    let matches = Command::new("Ccurl - custom curl")
        .about("It helps to make http methods")
        .version("1.0")
        .author("Praveen Chaudhary <chaudharypraveen98@gmail.com>")
        .arg(Arg::new("x-method").help("Http method which you want to use").long("x-method").short('X'))
        .arg(Arg::new("url").default_value(""))
        .get_matches();
    if let Some(url) = matches.get_one::<String>("url") {
        println!("Value for -c: {url}");
    }
    if let Some(method) = matches.get_one::<String>("x-method") {
        println!("Value for -method: {method}");
    }
}

