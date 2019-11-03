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
    pub name: String
}

#[derive(Identifiable, Queryable, Associations, Insertable)]
#[table_name="artist_genres"]
#[belongs_to(Artist)]
#[belongs_to(Genre)]
pub struct ArtistGenres {
    pub id: i32,
    pub artist_id: i32,
    pub genre_id: i32
}


