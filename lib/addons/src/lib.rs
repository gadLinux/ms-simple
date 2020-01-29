#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
pub mod schema;
pub mod person;
pub mod database;
pub mod repository;


































