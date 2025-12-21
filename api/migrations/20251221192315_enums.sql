-- Add migration script here
CREATE TYPE user_role AS ENUM ('client', 'freelancer');

CREATE TYPE country_enum AS ENUM (
  'US', 'CA', 'GB', 'AU', 'DE', 'FR', 'IN', 'JP', 'CN', 'BR', 'ZA', 'NG', 'KE', 'EG', 'MX', 'PK', 'RU', 'IT', 'ES', 'NL'
);
