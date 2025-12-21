-- Add migration script here

CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  username TEXT UNIQUE,
  phone_number TEXT NOT NULL,
  country country_enum NOT NULL,
  picture_url TEXT,
  role user_role NOT NULL DEFAULT 'freelancer',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
