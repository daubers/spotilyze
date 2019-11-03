extern crate dotenv;

use std::convert::TryInto;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use super::models::*;
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

    //println!("{:#?}", currently_playing.item.expect("No item").artists)
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
    let connection = establish_connection();
    for simple_artist in simple_artists {
        use super::schema::artist::dsl::*;
        use super::schema::artist;
        let full_artist = spotify.artist(simple_artist.id.as_ref().expect("No id")).expect("No");
        println!("{:#?}", full_artist);
        // Check to see if this artist is already in the database
        let result = artist.filter(uri.eq(&full_artist.uri)).load::<Artist>(&connection).expect("Couldn't load");
        println!("{:#?}", result.len());
        if result.len() == 0 {
            let new_artist = NewArtist {
                name: Some(full_artist.name),
                popularity: Some(full_artist.popularity.try_into().expect("Can't convert")),
                type_: Some(full_artist._type.as_str().to_string()),
                uri: Some(full_artist.uri)
            };
            diesel::insert_into(artist::table).values(&new_artist).execute(&connection).expect("Oops savinf new artist");
        }
    }
}