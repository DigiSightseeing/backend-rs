CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
  IF NOT EXISTS "User" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    username VARCHAR(255) UNIQUE NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    date_joined TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
  );

CREATE TABLE
  IF NOT EXISTS "Address" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    street VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    state VARCHAR(255) NOT NULL,
    zip_code VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "Coordinates" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    latitude VARCHAR(255) NOT NULL,
    longitude VARCHAR(255) NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "WorkingTime" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    start_time_weekdays TIMESTAMP NOT NULL,
    end_time_weekdays TIMESTAMP NOT NULL,
    start_time_weekends TIMESTAMP NOT NULL,
    end_time_weekends TIMESTAMP NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "TourLocationTag" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(255) NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "TourLocationTags" (
    location_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    PRIMARY KEY (location_id, tag_id),
    FOREIGN KEY (location_id) REFERENCES "Address" (id),
    FOREIGN KEY (tag_id) REFERENCES "TourLocationTag" (id)
  );

CREATE TABLE
  IF NOT EXISTS "Location" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    address_id UUID NOT NULL,
    coordinates_id UUID NOT NULL,
    working_time_id UUID NOT NULL,
    location_id VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    FOREIGN KEY (address_id) REFERENCES "Address" (id),
    FOREIGN KEY (coordinates_id) REFERENCES "Coordinates" (id),
    FOREIGN KEY (working_time_id) REFERENCES "WorkingTime" (id)
  );

CREATE TABLE
  IF NOT EXISTS "Tour" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    tag_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    tour_time_date TIMESTAMP NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "TourLocations" (
    tour_id UUID NOT NULL,
    location_id UUID NOT NULL,
    PRIMARY KEY (tour_id, location_id),
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id),
    FOREIGN KEY (location_id) REFERENCES "Location" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourUsers" (
    tour_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (tour_id, user_id),
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id),
    FOREIGN KEY (user_id) REFERENCES "User" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourReview" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    tour_id UUID NOT NULL,
    user_id UUID NOT NULL,
    rating INT NOT NULL,
    comment VARCHAR(255) NOT NULL,
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id),
    FOREIGN KEY (user_id) REFERENCES "User" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourReviews" (
    tour_id UUID NOT NULL,
    review_id UUID NOT NULL,
    PRIMARY KEY (tour_id, review_id),
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id),
    FOREIGN KEY (review_id) REFERENCES "TourReview" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourTag" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(255) NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "TourTags" (
    tour_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    PRIMARY KEY (tour_id, tag_id),
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id),
    FOREIGN KEY (tag_id) REFERENCES "TourTag" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourLocationReview" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    location_id UUID NOT NULL,
    user_id UUID NOT NULL,
    rating INT NOT NULL,
    comment VARCHAR(255) NOT NULL,
    FOREIGN KEY (location_id) REFERENCES "Location" (id),
    FOREIGN KEY (user_id) REFERENCES "User" (id)
  );

CREATE TABLE
  IF NOT EXISTS "TourLocationReviews" (
    location_id UUID NOT NULL,
    review_id UUID NOT NULL,
    PRIMARY KEY (location_id, review_id),
    FOREIGN KEY (location_id) REFERENCES "Location" (id),
    FOREIGN KEY (review_id) REFERENCES "TourReview" (id)
  );

CREATE TABLE
  IF NOT EXISTS "Role" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(255) NOT NULL,
    is_admin BOOLEAN NOT NULL
  );

CREATE TABLE
  IF NOT EXISTS "UserRoles" (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (role_id) REFERENCES "Role" (id)
  );

CREATE TABLE
  IF NOT EXISTS "Seen" (
    user_id UUID NOT NULL,
    tour_id UUID NOT NULL,
    PRIMARY KEY (user_id, tour_id),
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (tour_id) REFERENCES "Tour" (id)
  );
