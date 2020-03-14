CREATE TABLE pastes(
	pasteid varchar(32) PRIMARY KEY NOT NULL,
	token varchar(32) NOT NULL,
	lexer varchar(512),
	expiration timestamp NOT NULL,
	burn int NOT NULL,
	paste text NOT NULL,
	paste_lexed text,
	size int,
	lines int,
	sloc int
);