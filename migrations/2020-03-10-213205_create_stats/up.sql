CREATE TABLE stats (
	metric varchar(32) PRIMARY KEY NOT NULL,
	counter bigint NOT NULL
);
INSERT INTO stats (metric, counter) VALUES ('totalpastes', 0);
INSERT INTO stats (metric, counter) VALUES ('totalviews', 0);