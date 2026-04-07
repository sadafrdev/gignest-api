-- Add migration script here

CREATE TABLE users (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  username TEXT UNIQUE,
  phone_number TEXT NOT NULL,
  country country NOT NULL,
  role user_role NOT NULL DEFAULT 'Freelancer',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE otps (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  email TEXT NOT NULL,
  otp_hash TEXT NOT NULL,
  purpose TEXT NOT NULL CHECK (purpose IN ('password_reset')),
  expires_at TIMESTAMPTZ NOT NULL,
  used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE freelancers (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  picture_url TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE educations (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  country country NOT NULL,
  degree TEXT NOT NULL,
  institute TEXT NOT NULL,
  major TEXT NOT NULL,
  year_of_graduation DATE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE languages (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  language language NOT NULL ,
  language_level language_level NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE certificates (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  name TEXT NOT NULL ,
  certificate_by TEXT NOT NULL,
  year DATE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE portfolios (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  tagline TEXT NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE skills (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  skill skills_enum NOT NULL 
);

CREATE TABLE jobs (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  client_id BIGINT REFERENCES users(id),
  title TEXT NOT NULL ,
  description TEXT NOT NULL,
  budget_min NUMERIC NOT NULL,
  budget_max NUMERIC NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE proposals (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  freelancer_id BIGINT REFERENCES users(id),
  status proposal_status NOT NULL DEFAULT 'Pending',
  job_id BIGINT REFERENCES jobs(id),
  cover_letter TEXT NOT NULL,
  bid_amount NUMERIC(10, 2) NOT NULL,
  job_type job_type NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE contract (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
  freelancer_id BIGINT REFERENCES users(id)  NOT NULL,
  job_id BIGINT REFERENCES jobs(id)  NOT NULL,
  client_id BIGINT REFERENCES users(id)  NOT NULL,
  status contract_status NOT NULL DEFAULT 'Active',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE reviews (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  reviewer_id BIGINT REFERENCES users(id),
  reviewee_id BIGINT REFERENCES users(id),
  contract_id BIGINT REFERENCES contract(id)
  rating INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
  comment TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
