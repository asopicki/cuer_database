#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use crate::models::{Playlist, Cuecard};
use diesel::expression::AsExpression;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
	SqliteConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn playlist_by_uuid(u: &String, connection: &SqliteConnection) -> QueryResult<Playlist> {
	use crate::schema::playlists::dsl::*;

	playlists.filter(uuid.eq(u)).first::<Playlist>(connection)
}

pub fn cuecard_by_uuid(u: &String, connection: &SqliteConnection) -> QueryResult<Cuecard> {
	use crate::schema::cuecards::dsl::*;

	cuecards.filter(uuid.eq(u)).first::<Cuecard>(connection)
}

diesel_infix_operator!(CdMatch, " MATCH ");


// Normally you would put this on a trait instead
pub fn cd_match<T, U>(left: T, right: U) -> CdMatch<T, U::Expression> where
	T: Expression,
	U: AsExpression<T::SqlType>,
{
	CdMatch::new(left, right.as_expression())
}

//let users_with_name = users.select(id).filter(cd_match(name, "Sean"));