CREATE TABLE IF NOT EXISTS artist (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT,
    "popularity" INTEGER,
    "type" TEXT,
    "uri" TEXT
);


CREATE TABLE IF NOT EXISTS genre(
    "id" SERIAL PRIMARY KEY,
    "name" TEXT
    );

CREATE TABLE IF NOT EXISTS artist_genres (
    "id" SERIAL PRIMARY KEY,
    "artist_id" INTEGER REFERENCES artist(id),
    "genre_id" INTEGER REFERENCES genre(id)
);


CREATE TABLE IF NOT EXISTS album (
    "id" SERIAL PRIMARY KEY,
    "type" TEXT,
    "spotify_id" TEXT,
    "href" TEXT,
    "name" TEXT,
    "popularity" INTEGER,
    "release_date" TEXT,
    "uri" TEXT
);

CREATE TABLE IF NOT EXISTS track(
    "id" SERIAL PRIMARY KEY,
    "album" INTEGER REFERENCES album(id),
    "artists" INTEGER REFERENCES artist(id),
    "disc_number" INTEGER,
    "duration_ms" INTEGER,
    "explicit" BOOLEAN,
    "href" TEXT,
    "spotify_id" TEXT,
    "is_playable" BOOLEAN,
    "linked_from" INTEGER REFERENCES track(id),
    "name" TEXT,
    "popularity" INTEGER,
    "track_number" INTEGER,
    "type" TEXT,
    "uri" TEXT,
    "is_local" BOOLEAN
);

CREATE TABLE IF NOT EXISTS album_artists (
    "id" SERIAL PRIMARY KEY,
    "album_id" INTEGER REFERENCES album(id),
    "artist_id" INTEGER REFERENCES artist(id)
    );


CREATE TABLE IF NOT EXISTS album_genres (
    "id" SERIAL PRIMARY KEY,
    "album_id" INTEGER REFERENCES album(id),
    "genre_id" INTEGER REFERENCES genre(id)
    );

CREATE TABLE IF NOT EXISTS album_tracks (
    "id" SERIAL PRIMARY KEY,
    "album_id" INTEGER REFERENCES album(id),
    "track_id" INTEGER REFERENCES track(id)
    );