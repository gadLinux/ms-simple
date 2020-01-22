#[macro_use] extern crate log;

extern crate config;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand, ArgMatches};

use std::path::Path;
use std::collections::HashMap;


use config::{File,Config};


pub mod error;


pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "1.0.0";

pub struct BaseService {
    pub config: Option<Config>,
}

impl BaseService {
    fn setup(&self, matches: ArgMatches) -> Option<bool> {

     // Gets a value for config if supplied by user, or defaults to "default.conf"
        let config_filename = format!("{}.yml", self.config.as_ref().unwrap().get_str("application.name").unwrap_or("application".to_string()));
        let config = matches.value_of("config").unwrap_or(&config_filename);
        debug!("System initializing with config file {}", config);
        // It must be mutable
        //self.config.as_ref();.unwrap().merge(File::from(Path::new(config))).unwrap();
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
        Some(true)
    }

    fn configure(filename: &str) -> Option<Config> {
        // Gather all conf files from conf/ manually
        let mut settings = Config::default();

        settings
        .merge(File::from(Path::new(filename))).unwrap();
        Some(settings)
    }
}

pub trait Service {
   fn new(matches: ArgMatches) -> Self;
}


impl Service for BaseService {
    fn new(matches: ArgMatches) -> Self {

        let service = BaseService {
            config: BaseService::configure("application.yml"),
        };

        service.setup(matches);
        service
    }
}




































