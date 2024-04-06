CREATE TABLE Blog (
id BIGSERIAL NOT NULL PRIMARY KEY,
"name" TEXT NOT NULL
);
CREATE TABLE Post (
id SERIAL NOT NULL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL,
published BOOLEAN NOT NULL,
blog BIGINT NOT NULL,
byline TEXT 
);
CREATE TABLE Post_tags_Many (
owner INTEGER NOT NULL,
has TEXT NOT NULL
);
CREATE TABLE Tag (
tag TEXT NOT NULL PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS butane_migrations (
"name" TEXT NOT NULL PRIMARY KEY
);
