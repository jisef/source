-- Your SQL goes here
CREATE TABLE tracker.public.entry (
    id date PRIMARY KEY,
    amount float NOT NULL,
    category TEXT NOT NULL,
    description TEXT
);
