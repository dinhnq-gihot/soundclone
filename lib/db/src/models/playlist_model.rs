use {
    super::user_model::User, chrono::{DateTime, Local}, diesel::prelude::*, std::fmt::Debug
};

#[derive(
    Debug, Queryable, Identifiable, AsChangeset, Selectable, PartialEq, Associations, Clone,
)]
#[diesel(table_name = crate::schema::playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = user_id))]
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
