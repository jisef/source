-- Your SQL goes here
CREATE TABLE ausgabe (
   id SERIAL PRIMARY KEY,
   amount float NOT NULL,
   category VARCHAR NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
