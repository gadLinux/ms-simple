#![warn(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use] extern crate rocket;
//use super::schema::people;

extern crate rocket_contrib;
extern crate serde;

pub mod handler;
pub mod router;

use serde::{Serialize, Deserialize};
//pub mod repository;



//#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
//#[table_name = "people"]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

