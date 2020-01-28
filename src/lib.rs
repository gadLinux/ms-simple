#![warn(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]


use std::path::Path;
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;

#[macro_use] extern crate log;

extern crate rocket_contrib;
#[macro_use] extern crate rocket;



use rocket::Rocket;
use rocket::config::{Environment};
use rocket::error::{LaunchError};
extern crate config;
use config::{File,Config};


extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};

extern crate rocket_prometheus;
use rocket_prometheus::PrometheusMetrics;

/*
extern crate summer_addons;
use summer_addons::database;
*/
pub mod error;
pub mod handler; // TODO Move from here
pub mod external; // TODO Move from here


use external::HitCount;



pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "1.0.0";

pub struct BaseService<'a> {
    pub config: Option<Config>,
    matches: ArgMatches<'a>,
    pub server: rocket::Rocket,
    prometheus: Option<PrometheusMetrics>,
}

impl<'a> BaseService<'a> {
    fn setup(configuration: &Config, matches: &ArgMatches) -> Rocket {
         // Gets a value for config if supplied by user, or defaults to "default.conf"
        let config_filename = format!("{}.yml", configuration.get_str("application.name").unwrap_or("application".to_string()));
        let config_def_filename = matches.value_of("config").unwrap_or(&config_filename);
        debug!("Should load and merge a new config file {}", config_def_filename);
        // It must be mutable
        //configuration.as_ref();.unwrap().merge(File::from(Path::new(config))).unwrap();
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


        // Link to the custom config
        // Basic bootstrap
        let rocket_config = match rocket::config::Config::build(Environment::Staging)
    //            .address("1.2.3.4")
            .port(8080)
            .finalize() {
                Ok(config) => config,
                Err(e) => panic!("Error configuring {}", e),
            };
        // Do all internal setup and ignite()
        Rocket::custom(rocket_config)
    }

    fn read_configuration(filename: &str) -> Option<Config> {
        // Gather all conf files from conf/ manually
        let mut settings = Config::default();

        settings
            .merge(File::from(Path::new(filename))).unwrap();

        Some(settings)
    }

    pub fn launch(self, routes: Vec<rocket::Route>) -> LaunchError {
        let config = self.config.as_ref().unwrap();
        self.server
            .attach(self.prometheus.as_ref().unwrap().clone()) // TODO should be optional
            .manage(HitCount { count: AtomicI32::new(0) })
//            .manage(database::init_pool()) // TODO Get it right
            .mount(
                &format!("{}/{}",
                    config.get_str("server.context-path").unwrap_or("/api".to_string()),
                    config.get_str("application.version").unwrap_or("v1".to_string())),
                routes,)
            .mount(&format!("{}/{}/metrics",
                    config.get_str("server.context-path").unwrap_or("/api".to_string()),
                    config.get_str("application.version").unwrap_or("v1".to_string())),
                self.prometheus.unwrap())
            .launch()
    }
}

pub trait Service<'a> {
   fn new(matches: ArgMatches<'a>) -> Self;
}


impl<'a> Service<'a> for BaseService<'a> {
    fn new(matches: ArgMatches<'a>) -> Self {
        debug!("Initializing server engine");
        let custom_config = BaseService::read_configuration("application.yml");
        let rocket = BaseService::setup(&custom_config.as_ref().unwrap(), &matches);

        let prometheus = Some(PrometheusMetrics::new()); // TODO Should be able to disable

        let service = BaseService {
            config: custom_config,
            matches: matches,
            server: rocket,
            prometheus: prometheus,
        };
        service
    }
}




































