table! {
    poll_log (id) {
        id -> Int4,
        insert_time -> Nullable<Timestamp>,
        uri -> Varchar,
        sent_data -> Nullable<Json>,
        return_code -> Nullable<Int4>,
    }
}
