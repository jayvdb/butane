CREATE TABLE Blog (
id BYTEA NOT NULL PRIMARY KEY,
"name" TEXT NOT NULL
);
CREATE TABLE Post (
id BYTEA NOT NULL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL,
published BOOLEAN NOT NULL,
blog BYTEA NOT NULL,
byline TEXT ,
likes INTEGER NOT NULL
);
CREATE TABLE Post_tags_Many (
owner BYTEA NOT NULL,
has TEXT NOT NULL
);
CREATE TABLE Tag (
tag TEXT NOT NULL PRIMARY KEY
);
ALTER TABLE Post ADD FOREIGN KEY (blog) REFERENCES Blog(id);
ALTER TABLE Post_tags_Many ADD FOREIGN KEY (owner) REFERENCES Post(id);
ALTER TABLE Post_tags_Many ADD FOREIGN KEY (has) REFERENCES Tag(tag);
CREATE TABLE IF NOT EXISTS butane_migrations (
"name" TEXT NOT NULL PRIMARY KEY
);
