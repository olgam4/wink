CREATE TABLE IF NOT EXISTS winks (
    name text NOT NULL PRIMARY KEY,
    url text NOT NULL,
    hit_counter integer NOT NULL DEFAULT 0
);
