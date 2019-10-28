extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rspotify::spotify::model::context::SimplifiedPlayingContext;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn log_currently_playing(currently_playing :Option<SimplifiedPlayingContext>) {
    match currently_playing {
        Some(this_playing) => {
            println!("{:#?}", this_playing.item.expect("Oops")); 
        }
        None => println!("None")
    };
}