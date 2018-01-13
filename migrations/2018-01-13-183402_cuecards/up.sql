-- Your SQL goes here
CREATE TABLE cuecards (
	id INTEGER NOT NULL PRIMARY KEY,
	uuid TEXT NOT NULL UNIQUE,
	phase TEXT NOT NULL,
	rhythm TEXT NOT NULL,
	title TEXT NOT NULL,
	steplevel TEXT NOT NULL,
	difficulty TEXT NOT NULL,
	meta TEXT NOT NULL,
	content TEXT NOT NULL
);

CREATE VIRTUAL TABLE cardindex USING fts4(title, choreographer, meta, content);