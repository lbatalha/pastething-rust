CREATE TABLE dailystats (
	date timestamp PRIMARY KEY NOT NULL,
	pastecount int NOT NULL DEFAULT 0,
	pasteviews int NOT NULL DEFAULT 0
);