CREATE TABLE
  User (
    "id" UUID PRIMARY KEY DEFAULT uuid_genereate_v4 (),
    "first_name " VARCHAR(255) NOT NULL,
    "last_name " VARCHAR(255) NOT NULL,
    "email " VARCHAR(255) UNIQUE NOT NULL,
    "password " VARCHAR(255) NOT NULL,
    "date_joined " TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "last_login " TIMESTAMP
  );

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
