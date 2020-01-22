#![warn(missing_docs)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

extern crate rust_summer as summer;

//use std::error::Error;
use clap::{Arg, App, SubCommand};

use summer::config::Configuration;
use summer::error::ConfigError;

static SERVICE_NAME: &'static str = "simple";

fn main() {
    let app = App::new("Simple Microservice in rust")
      .version("1.0")
      .author("Gonzalo Aguilar Delgado <gaguilar@level2crm.com>")
      .about("Simple example of a microservice in rust")
      .arg(Arg::with_name("config")
           .short("c")
           .long("config")
           .value_name("FILE")
           .help("Sets a custom config file")
           .takes_value(true))
/*
      .arg(Arg::with_name("input_file")
           .help("Sets the input file to use")
           .required(true)
           .index(1))
*/
     .arg(Arg::with_name("v")
           .short("v")
           .multiple(true)
           .help("Sets the level of verbosity"))
      .subcommand(SubCommand::with_name("test")
                  .about("controls testing features")
                  .version("1.3")
                  .author("Someone E. <someone_else@other.com>")
                  .arg(Arg::with_name("debug")
                      .short("d")
                      .help("print debug information verbosely")));

    let matches = app.get_matches();

    env_logger::init();

 // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config_filename = format!("{}.conf",SERVICE_NAME);
    let config = matches.value_of("config").unwrap_or(&config_filename);
    debug!("System initializing with config file {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
//    println!("Using input file: {}", matches.value_of("input_file").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => debug!("No verbose info"),
        1 => debug!("Some verbose info"),
        2 => debug!("Tons of verbose info"),
        3 | _ => debug!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            debug!("Printing debug info...");
        } else {
            debug!("Printing normally...");
        }
    }

    let config = match Configuration::new(&config_filename) {
        Ok(config) => println!("Should start test"),
        //server::router::create_routes(config),
        Err(ConfigError::NotFound) =>
            println!("Resource not found"),
        Err(ConfigError::Io(err)) => {
                println!("Cannot configure the app, cannot process config file [{}]: {}",config_filename, err);
                //app.print_help();
            },
        Err(ConfigError::Serde(err)) =>
            println!("Cannot configure the app, cannot parse [{}]: {}",config_filename,err),
    };

}
