table! {
    cuecards (id) {
        id -> Integer,
        uuid -> Text,
        phase -> Text,
        rhythm -> Text,
        title -> Text,
        steplevel -> Text,
        difficulty -> Text,
        choreographer -> Text,
        meta -> Text,
        content -> Text,
    }
}

table! {
    playlists (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
    }
}

table! {
    playlist_cuecards (id) {
        id -> Integer,
        playlist_id -> Integer,
        cuecard_id -> Integer,
    }
}

joinable!(playlist_cuecards -> playlists (playlist_id));
joinable!(playlist_cuecards -> cuecards (cuecard_id));

allow_tables_to_appear_in_same_query!(playlists, playlist_cuecards, cuecards);