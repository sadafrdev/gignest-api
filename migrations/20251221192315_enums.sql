-- Add migration script here

CREATE TYPE country AS ENUM (
  'us','ca','gb','au','de','fr','in','jp','cn','br','za','ng','ke','eg','mx','pk','ru','it','es','nl'
);

CREATE TYPE user_role AS ENUM ('client', 'freelancer');

CREATE TYPE language AS ENUM (
  "English",
  "Urdu",
  "Spanish",
  "Chinese",
  "Korean",
  "French",
  "Russian",
  "Germany",
  "Arabic",
  "Hindi",
  "Persian",
  "Turkish",
  "Bengali"
  );

CREATE TYPE language_level AS ENUM ( 
  "Begginer",
  "Intermediate",
  "Fluent",
);

CREATE TYPE skills_enum AS ENUM (
  'WEB_DEVELOPMENT',
  'APP_DEVELOPMENT',
  'DATA_SCIENCE',
  'CYBER_SECURITY',
  'CLOUD_COMPUTING',
  'RUST_PROGRAMMING',
  'PYTHON_PROGRAMMING',
  'JAVA_PROGRAMMING',
  'FRONTEND_DEVELOPMENT',
  'BACKEND_DEVELOPMENT',
  'HTML_CSS',
  'JAVASCRIPT',
  'MOBILE_DEVELOPMENT',
  'GRAPHIC_DESIGN',
  'DIGITAL_MARKETING',
  'CONTENT_WRITING',
  'DATA_ANALYSIS',
  'PROJECT_MANAGEMENT',
  'SEO_SPECIALIST',
  'VIDEO_EDITING',
  'UI_UX_DESIGN'
);
CREATE TYPE job_type AS ENUM ('Fixed', 'Hourly');

CREATE TYPE proposal_status AS ENUM ('Pending', 'Accepted', 'Rejected');
