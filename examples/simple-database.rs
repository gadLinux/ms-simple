#![warn(missing_docs)]
#![feature(decl_macro, proc_macro_hygiene)]


#[macro_use] extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App, SubCommand,ArgMatches};

#[macro_use]
extern crate rocket;

extern crate rust_summer as summer;

use summer::{BaseService, Service};


fn main() {

    // Build command line specs
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

    // Init logging
    env_logger::init();

    // Create the service
    let service: BaseService = Service::new(app.get_matches());
    debug!("Application {:?}",
     service.config.as_ref().unwrap().get_str("application.name").unwrap_or("Unknown".to_string()));

    let routes = routes![summer::handler::all,
                    summer::handler::get,
/*                    people::handler::post,
                    people::handler::put,
                    people::handler::delete
*/
                    ];
     service.launch(routes);
//    service.server.mount(&format!("/{}/{}", "app", "v1"),routes,).launch();


}
