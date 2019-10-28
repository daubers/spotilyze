use super::schema::*;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="artist"]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub popularity: i32,
    pub type_: String,
    pub uri: String
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="genre"]
pub struct Genre {
    pub id: i32,
    pub name: String
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="artist_genres"]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct ArtistGenres {
    pub id: i32,
    pub artist_id: i32,
    pub genre_id: i32
}
