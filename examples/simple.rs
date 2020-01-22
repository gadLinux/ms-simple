#![warn(missing_docs)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;


extern crate rust_summer as summer;

use std::collections::HashMap;
//use std::error::Error;
use clap::{Arg, App, SubCommand,ArgMatches};

use summer::error::ConfigError;
use summer::{BaseService, Service};

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
                  .author("Gonzalo Aguilar Delgado<gaguilar@level2crm.com.com>")
                  .arg(Arg::with_name("debug")
                      .short("d")
                      .help("print debug information verbosely")));

    env_logger::init();


    let service: BaseService = Service::new(app.get_matches());
    debug!("Application {:?}",
         service.config.unwrap().get_str("application.name").unwrap_or("Unknown".to_string()));
    /*

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
    */

}
