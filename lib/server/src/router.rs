
use rocket;
//use connection;

use super::handler;
use super::config::{Configuration, Server, Application};
use std::sync::atomic::AtomicI32;
use rocket::config::{Config, Environment};
use std::convert::TryInto;

pub struct HitCount {
    pub count: AtomicI32
}


pub fn create_routes(config: Configuration) {
    let server = &config.server.unwrap_or(Server {
            port: 8080,
            context_path: String::from("service")
        });
    let app = &config.application.unwrap_or(Application {
        name: Some(String::from("Default")),
        version: String::from("v1")
    });
    let config = match Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(server.port.try_into().unwrap())
        .finalize() {
            Ok(config) => config,
            Err(e) => panic!("Error configuring {}", e),
        };

//    rocket::ignite()
//        .manage(connection::init_pool())
        rocket::custom(config)   
        .manage(HitCount { count: AtomicI32::new(0) })
        .mount(&format!("{}/{}", server.context_path, app.version),
               routes![handler::all,
/*                    people::handler::get,
                    people::handler::post,
                    people::handler::put,
                    people::handler::delete
*/                    
                    ],
        ).launch();
}
