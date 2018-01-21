use super::schema::cuecards;
use super::schema::playlists;
use super::schema::playlist_cuecards;
use diesel::prelude::*;
use diesel::{insert_into, replace_into, delete, SqliteConnection, QueryResult, RunQueryDsl,
	ExpressionMethods};


#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "cuecards"]
pub struct Cuecard {
	pub id: i32,
	pub uuid: String,
	pub phase: String,
	pub rhythm: String,
	pub title: String,
	pub steplevel: String,
	pub difficulty: String,
	pub choreographer: String,
	pub meta: String,
	pub content: String,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "cuecards"]
pub struct NewCuecard<'a> {
	pub uuid: &'a str,
	pub phase: &'a str,
	pub rhythm: &'a str,
	pub title: &'a str,
	pub steplevel: &'a str,
	pub difficulty: &'a str,
	pub choreographer: &'a str,
	pub meta: &'a str,
	pub content: &'a str,
}

impl Cuecard {
	pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
		use schema::cuecards::dsl::*;

		delete(cuecards).filter(id.eq(self.id)).execute(conn)
	}
}

impl<'a> NewCuecard<'a> {
	/// Inserts the cuecard into the database, or updates an existing one.
	pub fn create_or_update(&self, conn: &SqliteConnection) -> QueryResult<usize> {
		use schema::cuecards::dsl::*;

		replace_into(cuecards)
			.values(self)
			.execute(conn)
	}
}


#[derive(Clone, Queryable, Identifiable, QueryableByName, Debug, Serialize, Deserialize)]
#[table_name = "playlists"]
pub struct Playlist {
	pub id: i32,
	pub uuid: String,
	pub name: String,
}

impl Playlist {
	pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
		use schema::playlists::dsl::*;

		delete(playlists).filter(id.eq(self.id)).execute(conn)
	}
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "playlists"]
pub struct NewPlaylist<'a> {
	pub uuid: &'a str,
	pub name: &'a str,
}

impl<'a> NewPlaylist<'a> {
	/// Inserts the cuecard into the database, or updates an existing one.
	pub fn create_or_update(&self, conn: &SqliteConnection) -> QueryResult<Playlist> {
		use schema::playlists::dsl::*;

		replace_into(playlists)
			.values(self)
			.execute(conn).unwrap();

		return playlists.filter(uuid.eq(self.uuid)).get_result(conn);
	}
}

#[derive(Clone, Queryable, Identifiable, Associations, QueryableByName, Debug, Serialize, Deserialize)]
#[belongs_to(Playlist)]
#[belongs_to(Cuecard)]
#[table_name = "playlist_cuecards"]
pub struct PlaylistCuecard {
	pub id: i32,
	pub playlist_id: i32,
	pub cuecard_id: i32,
}

impl PlaylistCuecard {
	pub fn delete(&self, conn: &SqliteConnection) -> QueryResult<usize> {
		use schema::playlist_cuecards::dsl::*;

		delete(playlist_cuecards).filter(id.eq(self.id)).execute(conn)
	}
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "playlist_cuecards"]
pub struct NewPlaylistCuecard<'a> {
	pub playlist_id: &'a i32,
	pub cuecard_id: &'a i32,
}

impl<'a> NewPlaylistCuecard<'a> {
	/// Inserts the cuecard into the database
	pub fn create(&self, conn: &SqliteConnection) -> QueryResult<usize> {
		use schema::playlist_cuecards::dsl::*;
		insert_into(playlist_cuecards).values(self).execute(conn)
	}
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Cardindex {
	pub rowid: i32,
	pub docid: i32,
	pub title: String,
	pub choreographer: String,
	pub meta: String,
	pub content: String
}