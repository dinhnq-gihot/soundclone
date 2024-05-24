use {
    chrono::{DateTime, Local, NaiveDate},
    diesel::prelude::*,
    std::fmt::Debug,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a DateTime<Local>,
}

#[derive(
    Debug, Queryable, Identifiable, AsChangeset, Selectable, PartialEq, Associations, Clone,
)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = artist_id))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist_id: i32,
    pub release_date: Option<NaiveDate>,
    pub cover_art: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum<'a> {
    pub title: &'a str,
    pub artist_id: &'a i32,
    pub release_date: Option<&'a NaiveDate>,
    pub cover_art: Option<&'a str>,
    pub created_at: &'a DateTime<Local>,
}

#[derive(Debug, Identifiable, AsChangeset, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub user_id: Option<i32>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlayist<'a> {
    pub name: &'a str,
    pub user_id: Option<&'a i32>,
    pub description: Option<&'a str>,
    pub is_public: Option<&'a bool>,
}

#[derive(Debug, Identifiable, AsChangeset, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Album))]
pub struct Track {
    pub id: i32,
    pub title: String,
    pub album_id: Option<i32>,
    pub artist_id: Option<i32>,
    pub duration: Option<i32>,
    pub file_path: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTrack<'a> {
    pub title: &'a str,
    pub album_id: Option<&'a i32>,
    pub artist_id: Option<&'a i32>,
    pub duration: Option<&'a i32>,
    pub file_path: Option<&'a str>,
}

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::playlists_tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Playlist))]
#[diesel(belongs_to(Track))]
#[diesel(primary_key(playlist_id, track_id))]
pub struct PlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32,
}

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::users_liked_tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Track))]
#[diesel(primary_key(user_id, track_id))]
pub struct UserLikedTrack {
    pub user_id: i32,
    pub track_id: i32,
}
