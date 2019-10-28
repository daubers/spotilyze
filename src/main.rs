
mod spotify;

#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod db;

use crate::spotify::main as spotimain;

fn main() {
    println!("Hello, world!");
    spotimain();
}
