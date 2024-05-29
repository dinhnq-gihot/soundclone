use {
    super::user_model::User,
    chrono::{DateTime, Local, NaiveDate},
    diesel::prelude::*,
    std::fmt::Debug,
};

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
