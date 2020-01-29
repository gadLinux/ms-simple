use std::sync::atomic::AtomicI32;

extern crate serde;
use self::serde::{Serialize, Deserialize};

pub struct HitCount {
    pub count: AtomicI32
}
