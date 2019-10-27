table! {
    album (id) {
        id -> Int4,
    }
}

table! {
    artist (id) {
        id -> Int4,
    }
}

table! {
    available_markets (id) {
        id -> Int4,
        CODE -> Nullable<Text>,
    }
}

table! {
    external_id (id) {
        id -> Int4,
    }
}

table! {
    external_url (id) {
        id -> Int4,
        key -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

table! {
    linked_track (id) {
        id -> Int4,
    }
}

table! {
    poll_log (id) {
        id -> Int4,
        insert_time -> Nullable<Timestamp>,
        uri -> Varchar,
        sent_data -> Nullable<Json>,
        return_code -> Nullable<Int4>,
    }
}

table! {
    restriction (id) {
        id -> Int4,
    }
}

table! {
    track (id) {
        id -> Int4,
        album -> Nullable<Int4>,
        artist -> Nullable<Int4>,
        available_markets -> Nullable<Int4>,
        disc_number -> Nullable<Int4>,
        duration_ms -> Nullable<Int4>,
        explicit -> Nullable<Bool>,
        external_ids -> Nullable<Int4>,
        external_url -> Nullable<Int4>,
        href -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        is_playable -> Nullable<Bool>,
        linked_from -> Nullable<Int4>,
        restrictions -> Nullable<Int4>,
        name -> Nullable<Text>,
        popularity -> Nullable<Int4>,
        preview_url -> Nullable<Text>,
        track_number -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        uri -> Nullable<Text>,
        is_local -> Nullable<Bool>,
    }
}

joinable!(track -> album (album));
joinable!(track -> artist (artist));
joinable!(track -> available_markets (available_markets));
joinable!(track -> external_id (external_ids));
joinable!(track -> external_url (external_url));
joinable!(track -> linked_track (linked_from));
joinable!(track -> restriction (restrictions));

allow_tables_to_appear_in_same_query!(
    album,
    artist,
    available_markets,
    external_id,
    external_url,
    linked_track,
    poll_log,
    restriction,
    track,
);
