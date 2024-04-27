
ALTER TABLE Post_tags_Many DROP CONSTRAINT Post_tags_Many_owner_fkey;
ALTER TABLE Post_tags_Many DROP CONSTRAINT Post_tags_Many_has_fkey;
DROP TABLE Tag;
DROP TABLE Post_tags_Many;
ALTER TABLE Post ADD COLUMN tags JSONB NOT NULL DEFAULT null;
