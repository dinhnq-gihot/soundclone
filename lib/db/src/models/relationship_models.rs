use {
    super::playlist_model::Playlist, super::track_model::Track, super::user_model::User,
    diesel::prelude::*, std::fmt::Debug,
};

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
