extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::context::SimplifiedPlayingContext;
use rspotify::spotify::client::Spotify;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn log_currently_playing(currently_playing :SimplifiedPlayingContext) {

    println!("{:#?}", currently_playing.item.expect("No item").artists)
}

pub fn get_currently_playing(spotify :Spotify) -> SimplifiedPlayingContext {
    let history = spotify.current_playing(None);
    let track_history = history.expect("Oops").expect("No track history");
    let item = track_history.clone().item.expect("No item");
    let artists = item.artists;
    process_simplified_artists(spotify, &artists);
    return track_history;
}

fn process_simplified_artists(spotify :Spotify, simple_artists: &Vec<SimplifiedArtist>) {
    for simple_artist in simple_artists {
            println!("{:#?}", spotify.artist(simple_artist.id.as_ref().expect("No id")).expect("No"))
    }
}