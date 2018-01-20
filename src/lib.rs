#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::{Playlist, Cuecard};

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn playlist_by_uuid(u: &String, connection: &SqliteConnection) -> QueryResult<Playlist> {
	use schema::playlists::dsl::*;

	playlists.filter(uuid.eq(u)).first::<Playlist>(connection)
}

pub fn cuecard_by_uuid(u: &String, connection: &SqliteConnection) -> QueryResult<Cuecard> {
	use schema::cuecards::dsl::*;

	cuecards.filter(uuid.eq(u)).first::<Cuecard>(connection)
}