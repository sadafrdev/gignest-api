-- Add migration script here

CREATE TYPE country AS ENUM (
  'us','ca','gb','au','de','fr','in','jp','cn','br','za','ng','ke','eg','mx','pk','ru','it','es','nl'
);

CREATE TYPE user_role AS ENUM ('Client', 'Freelancer');

CREATE TYPE language AS ENUM ('English', 'Urdu', 'Spanish', 'Chinese', 'Korean', 'French', 'Russian', 'German', 'Arabic', 'Hindi', 'Persian', 'Turkish', 'Bengali');

CREATE TYPE language_level AS ENUM ( 
  'Begginer',
  'Intermediate',
  'Fluent'
);

CREATE TYPE skills_enum AS ENUM (
  'WebDevelopment',
  'AppDevelopment',
  'DataScience',
  'CyberSecurity',
  'CloudComputing',
  'RustProgramming',
  'PythonProgramming',
  'JavaProgramming',
  'FrontendDevelopment',
  'BackendDevelopment',
  'HtmlCss',
  'Javascript',
  'MobileDevelopment',
  'GraphicDesign',
  'DigitalMarketing',
  'ContentWriting',
  'DataAnalysis',
  'ProjectManagement',
  'SeoSpeciallities',
  'VideoEditing',
  'UiUxDesign'
);

CREATE TYPE job_type AS ENUM ('Fixed', 'Hourly');

CREATE TYPE proposal_status AS ENUM ('Pending', 'Accepted', 'Rejected');

CREATE TYPE contract_status AS ENUM ('Active', 'Completed');
