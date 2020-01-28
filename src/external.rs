use std::sync::atomic::AtomicI32;

extern crate serde;
use self::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

pub struct HitCount {
    pub count: AtomicI32
}
