-- Your SQL goes here
CREATE TABLE public.entry (
    id date PRIMARY KEY,
    amount float NOT NULL,
    category TEXT NOT NULL,
    description TEXT
);
