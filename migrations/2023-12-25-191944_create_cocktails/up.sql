-- Your SQL goes here
CREATE TABLE cocktails (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  ingredients TEXT NOT NULL,
  recipe  TEXT NOT NULL
)
