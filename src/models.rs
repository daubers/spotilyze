use super::schema::*;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="artist"]
pub struct Artist {
    pub id: i32,
    pub name: Option<String>,
    pub popularity: Option<i32>,
    pub type_: Option<String>,
    pub uri: Option<String>
}

#[derive(Insertable)]
#[table_name="artist"]
pub struct NewArtist {
    pub name: Option<String>,
    pub popularity: Option<i32>,
    pub type_: Option<String>,
    pub uri: Option<String>
}


#[derive(Identifiable, Queryable, Associations, Insertable)]
#[table_name="genre"]
pub struct Genre {
    pub id: i32,
    pub name: Option<String>
}

#[derive(Insertable)]
#[table_name="genre"]
pub struct NewGenre {
    pub name: Option<String>
}

#[derive(Identifiable, Queryable, Associations, Insertable)]
#[table_name="artist_genres"]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct ArtistGenres {
    pub id: i32,
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>
}

#[derive(Insertable)]
#[table_name="artist_genres"]
pub struct NewArtistGenres {
    pub artist_id: Option<i32>,
    pub genre_id: Option<i32>
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="album"]
pub struct Album {
    pub id: i32,
    pub type_: Option<String>,
    pub spotify_id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub popularity: Option<i32>,
    pub release_date: Option<String>,
    pub uri: Option<String>
}

#[derive(Insertable)]
#[table_name="album"]
pub struct NewAlbum {
    pub type_: Option<String>,
    pub spotify_id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub popularity: Option<i32>,
    pub release_date: Option<String>,
    pub uri: Option<String>
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="album_artists"]
pub struct AlbumArtist {
    pub id: i32,
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>
}

#[derive(Insertable)]
#[table_name="album_artists"]
pub struct NewAlbumArtist {
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>
}