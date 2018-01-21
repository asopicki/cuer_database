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

table! {
	cardindex(rowid) {
		rowid -> Integer,
		docid -> Integer,
		title -> Text,
		choreographer -> Text,
		meta -> Text,
		content -> Text,
	}
}

joinable!(playlist_cuecards -> playlists (playlist_id));
joinable!(playlist_cuecards -> cuecards (cuecard_id));
joinable!(cardindex -> cuecards(docid));

allow_tables_to_appear_in_same_query!(playlists, playlist_cuecards, cuecards);
allow_tables_to_appear_in_same_query!(cardindex, cuecards);