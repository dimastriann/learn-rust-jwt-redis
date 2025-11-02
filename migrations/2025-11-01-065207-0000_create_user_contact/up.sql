-- Your SQL goes here
CREATE TABLE contacts
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    phone      VARCHAR(50),
    email      VARCHAR(255),
    mobile     VARCHAR(50),
    address    TEXT,
    city       VARCHAR(100),
    state      VARCHAR(100),
    zipcode    VARCHAR(20),
    country    VARCHAR(100),
    created_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(100) NOT NULL UNIQUE,
    password   VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    full_name  VARCHAR(255),
    contact_id INT          NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT fk_contact FOREIGN KEY (contact_id)
           REFERENCES contacts (id)
           ON DELETE CASCADE
);
