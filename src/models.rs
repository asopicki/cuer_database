#[derive(Queryable)]
pub struct Cuecard {
	pub id: i32,
	pub uuid: String,
	pub phase: String,
	pub rhythm: String,
	pub title: String,
	pub steplevel: String,
	pub difficulty: String,
	pub meta: String,
	pub content: String,
}