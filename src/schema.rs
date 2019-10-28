table! {
    album (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        href -> Nullable<Text>,
        name -> Nullable<Text>,
        popularity -> Nullable<Int4>,
        release_date -> Nullable<Text>,
        uri -> Nullable<Text>,
    }
}

table! {
    album_artists (id) {
        id -> Int4,
        album_id -> Nullable<Int4>,
        artist_id -> Nullable<Int4>,
    }
}

table! {
    album_genres (id) {
        id -> Int4,
        album_id -> Nullable<Int4>,
        genre_id -> Nullable<Int4>,
    }
}

table! {
    album_tracks (id) {
        id -> Int4,
        album_id -> Nullable<Int4>,
        track_id -> Nullable<Int4>,
    }
}

table! {
    artist (id) {
        id -> Int4,
        name -> Nullable<Text>,
        popularity -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        uri -> Nullable<Text>,
    }
}

table! {
    artist_genres (id) {
        id -> Int4,
        artist_id -> Nullable<Int4>,
        genre_id -> Nullable<Int4>,
    }
}

table! {
    genre (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

table! {
    track (id) {
        id -> Int4,
        album -> Nullable<Int4>,
        artists -> Nullable<Int4>,
        disc_number -> Nullable<Int4>,
        duration_ms -> Nullable<Int4>,
        explicit -> Nullable<Bool>,
        href -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        is_playable -> Nullable<Bool>,
        linked_from -> Nullable<Int4>,
        name -> Nullable<Text>,
        popularity -> Nullable<Int4>,
        track_number -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        uri -> Nullable<Text>,
        is_local -> Nullable<Bool>,
    }
}

joinable!(album_artists -> album (album_id));
joinable!(album_artists -> artist (artist_id));
joinable!(album_genres -> album (album_id));
joinable!(album_genres -> genre (genre_id));
joinable!(album_tracks -> album (album_id));
joinable!(album_tracks -> track (track_id));
joinable!(artist_genres -> artist (artist_id));
joinable!(artist_genres -> genre (genre_id));
joinable!(track -> album (album));
joinable!(track -> artist (artists));

allow_tables_to_appear_in_same_query!(
    album,
    album_artists,
    album_genres,
    album_tracks,
    artist,
    artist_genres,
    genre,
    track,
);
