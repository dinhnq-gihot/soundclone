// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        title -> Varchar,
        artist_id -> Nullable<Int4>,
        release_date -> Nullable<Date>,
        cover_art -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    playlists (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Nullable<Int4>,
        description -> Nullable<Text>,
        is_public -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    playlists_tracks (playlist_id, track_id) {
        playlist_id -> Int4,
        track_id -> Int4,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        title -> Varchar,
        album_id -> Nullable<Int4>,
        artist_id -> Nullable<Int4>,
        duration -> Nullable<Int4>,
        file_path -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_picture -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users_liked_tracks (user_id, track_id) {
        user_id -> Int4,
        track_id -> Int4,
    }
}

diesel::joinable!(albums -> users (artist_id));
diesel::joinable!(playlists -> users (user_id));
diesel::joinable!(playlists_tracks -> playlists (playlist_id));
diesel::joinable!(playlists_tracks -> tracks (track_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> users (artist_id));
diesel::joinable!(users_liked_tracks -> tracks (track_id));
diesel::joinable!(users_liked_tracks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    playlists,
    playlists_tracks,
    tracks,
    users,
    users_liked_tracks,
);
