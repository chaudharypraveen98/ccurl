use clap::{Arg, ArgMatches, Command};

pub fn get_arguments()-> ArgMatches{
    Command::new("Ccurl - custom curl")
        .about("It helps to make http methods")
        .version("1.0")
        .disable_version_flag(true)
        .author("Praveen Chaudhary <chaudharypraveen98@gmail.com>")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("x-method")
                .help("Http method which you want to use")
                .long("x-method")
                .short('X'),
        )
        .arg(
            Arg::new("data")
                .help("Payload you want to send with the request")
                .long("data")
                .short('d'),
        )
        .arg(
            Arg::new("headers")
                .help("Request header")
                .long("header")
                .short('H'),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}