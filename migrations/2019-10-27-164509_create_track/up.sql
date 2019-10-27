CREATE TABLE IF NOT EXISTS external_url("id" SERIAL PRIMARY KEY, "key" TEXT, "value" TEXT);

CREATE TABLE IF NOT EXISTS available_markets("id" SERIAL PRIMARY KEY, "CODE" TEXT);

CREATE TABLE IF NOT EXISTS external_id("id" SERIAL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS linked_track("id" SERIAL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS restriction("id" SERIAL PRIMARY KEY);



CREATE TABLE IF NOT EXISTS artist (
    "id" SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS album (
    "id" SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS track(
    "id" SERIAL PRIMARY KEY,
    "album" INTEGER REFERENCES album(id),
    "artist" INTEGER REFERENCES artist(id),
    "available_markets" INTEGER REFERENCES available_markets(id),
    "disc_number" INTEGER,
    "duration_ms" INTEGER,
    "explicit" BOOLEAN,
    "external_ids" INTEGER REFERENCES external_id(id),
    "external_url" INTEGER REFERENCES external_url(id),
    "href" TEXT,
    "spotify_id" TEXT,
    "is_playable" BOOLEAN,
    "linked_from" INTEGER REFERENCES linked_track(id),
    "restrictions" INTEGER REFERENCES restriction(id),
    "name" TEXT,
    "popularity" INTEGER,
    "preview_url" TEXT,
    "track_number" INTEGER,
    "type" TEXT,
    "uri" TEXT,
    "is_local" BOOLEAN
);