-- Your SQL goes here
CREATE TABLE product_categories
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE products
(
    id          SERIAL PRIMARY KEY,
    name        TEXT             NOT NULL,
    sale_price  DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    cost        DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    stock       DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    description TEXT,
    category_id INTEGER          NOT NULL REFERENCES product_categories (id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ      NOT NULL DEFAULT CURRENT_TIMESTAMP
);
