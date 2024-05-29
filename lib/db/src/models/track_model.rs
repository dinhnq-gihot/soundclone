use {
    super::{album_model::Album, user_model::User},
    chrono::{DateTime, Local},
    diesel::prelude::*,
    std::fmt::Debug,
};

#[derive(
    Debug, Queryable, Identifiable, AsChangeset, Selectable, PartialEq, Associations, Clone,
)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = artist_id))]
#[diesel(belongs_to(Album, foreign_key = album_id))]
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
    pub created_at: Option<&'a DateTime<Local>>,
}
