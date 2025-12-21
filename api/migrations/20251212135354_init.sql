-- Add migration script here
CREATE TYPE user_role AS ENUM ('client', 'freelancer');

CREATE TYPE country_enum AS ENUM (
  'us','ca','gb','au','de','fr','in','jp','cn','br','za','ng','ke','eg','mx','pk','ru','it','es','nl'
);

CREATE TABLE users (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
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
