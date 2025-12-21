-- Add migration script here

CREATE TYPE country AS ENUM (
  'us','ca','gb','au','de','fr','in','jp','cn','br','za','ng','ke','eg','mx','pk','ru','it','es','nl'
);

CREATE TYPE user_role AS ENUM ('client', 'freelancer');
