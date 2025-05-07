-- Your SQL goes here
CREATE TABLE contacts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  files VARCHAR
)