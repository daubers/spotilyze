extern crate dotenv;

use std::convert::TryInto;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use super::models::*;
use std::env;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::context::SimplifiedPlayingContext;
use rspotify::spotify::model::album::SimplifiedAlbum;
use rspotify::spotify::client::Spotify;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn log_currently_playing(currently_playing :SimplifiedPlayingContext) {

    //println!("{:#?}", currently_playing.item.expect("No item").album)
}

pub fn get_currently_playing(spotify :Spotify) -> SimplifiedPlayingContext {
    let history = spotify.current_playing(None);
    let track_history = history.expect("Oops").expect("No track history");
    let item = track_history.clone().item.expect("No item");
    let artists = item.artists;
    let connection = establish_connection();
    process_simplified_artists(&spotify, &artists, &connection);
    process_album(&spotify, item.album, &connection);
    return track_history;
}

fn process_genres(genres: &Vec<String>, connection: &PgConnection) -> Vec<i32> {
    use super::schema::genre::dsl::*;
    use super::schema::genre;
    let mut return_list: Vec<i32> = Vec::new();
    for this_genre in genres {
        let result = genre.filter(name.eq(this_genre)).load::<Genre>(connection).expect("Couldn't load");
        if result.len() == 0 {
            let new_genre = NewGenre {
                name: Some(this_genre.to_string())
            };
            println!("{:?}", this_genre);
            let insert_id = diesel::insert_into(genre::table).values(&new_genre).returning(id).get_result::<i32>(connection).expect("Oops saving new genre");
            return_list.push(insert_id);
        } else {
            return_list.push(result[0].id);
        }
    };
    return_list
}

fn link_artist_to_genres(this_artist_id: i32, genre_ids: Vec<i32>, connection: &PgConnection){
    use super::schema::artist_genres::dsl::*;
    use super::schema::artist_genres;
    for this_genre in genre_ids {
        let result = artist_genres.filter(genre_id.eq(this_genre)).load::<ArtistGenres>(connection).expect("Couldn't load");
        if result.len() == 0 {
            let new_link = NewArtistGenres {
                artist_id: Some(this_artist_id),
                genre_id: Some(this_genre)
            };
            let _link_id = diesel::insert_into(artist_genres::table).values(&new_link).returning(id).get_result::<i32>(connection).expect("Oops savinf new artist to genre link");

        }
    }
}

fn process_album(spotify: &Spotify, simple_album: SimplifiedAlbum, connection: &PgConnection){
    use super::schema::album::dsl::*;
    use super::schema::album;
    let result = album.filter(spotify_id.eq(simple_album.id.as_ref().expect("No id"))).load::<Album>(connection).expect("Couldn't load");
    let album_id = if result.len() > 0 {
        result[0].id
    }
    else {
        let this_album = NewAlbum {
            type_: simple_album.album_type,
            spotify_id: simple_album.id.clone(),
            href: simple_album.href,
            name: Some(simple_album.name),
            release_date: simple_album.release_date,
            uri: simple_album.uri,
            popularity: None
        };
        let album_id = diesel::insert_into(album::table).values(&this_album).returning(id).get_result::<i32>(connection).expect("Oops savinf new artist");
        album_id        
    };
    let artist_ids = process_simplified_artists(spotify,  &simple_album.artists, connection);
    println!("album_artists: {:#?}", artist_ids);
    link_album_to_artist(artist_ids, album_id, connection);
    let full_album = spotify.album(simple_album.id.expect("Ok").as_str()).expect("Ok");
    println!("full_album: {:#?}", full_album.genres);
    let genre_ids = process_genres(&full_album.genres, connection);
    println!("full_album_genre_ids: {:#?}", genre_ids);


}

fn link_album_to_artist(artist_ids: Vec<i32>, this_album_id: i32, connection: &PgConnection){
    use super::schema::album_artists::dsl::*;
    use super::schema::album_artists;
    for this_artist_id in artist_ids {
        let result = album_artists.filter(artist_id.eq(this_artist_id)).load::<AlbumArtist>(connection).expect("Couldn't load");
        if result.len() == 0 {
            let new_link = NewAlbumArtist {
                artist_id: Some(this_artist_id),
                album_id: Some(this_album_id)
            };
            let _link_id = diesel::insert_into(album_artists::table).values(&new_link).returning(id).get_result::<i32>(connection).expect("Oops saving new album tp artist link");
        };
    }
}

fn process_simplified_artists(spotify: &Spotify, simple_artists: &Vec<SimplifiedArtist>, connection: &PgConnection) -> Vec<i32> {
    use super::schema::artist::dsl::*;
    use super::schema::artist;
    let mut return_list: Vec<i32> = Vec::new();
    for simple_artist in simple_artists {
        let full_artist = spotify.artist(simple_artist.id.as_ref().expect("No id")).expect("No");
        println!("{:#?}", full_artist);
        // Check to see if this artist is already in the database
        let result = artist.filter(uri.eq(&full_artist.uri)).load::<Artist>(connection).expect("Couldn't load");
        println!("{:#?}", result.len());
        let artist_id;
        if result.len() == 1{
            artist_id = result[0].id;
        }
        else {
            let new_artist = NewArtist {
                name: Some(full_artist.name),
                popularity: Some(full_artist.popularity.try_into().expect("Can't convert")),
                type_: Some(full_artist._type.as_str().to_string()),
                uri: Some(full_artist.uri)
            };
            artist_id = diesel::insert_into(artist::table).values(&new_artist).returning(id).get_result::<i32>(connection).expect("Oops savinf new artist");
        }
        let genre_ids = process_genres(&full_artist.genres, &connection);
        println!("{:#?}", artist_id);
        println!("{:#?}", genre_ids);
        link_artist_to_genres(artist_id, genre_ids, &connection);
        return_list.push(artist_id);
    }
    return_list
}