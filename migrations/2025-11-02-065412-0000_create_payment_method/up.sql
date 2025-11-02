-- Your SQL goes here
CREATE TABLE payment_methods
(
    id         SERIAL PRIMARY KEY,
    name       TEXT        NOT NULL,
    code       TEXT        NOT NULL UNIQUE,
    is_cash    BOOLEAN,
    is_bank    BOOLEAN,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        CONSTRAINT chk_is_cash_or_is_bank
            CHECK (
                (is_cash IS TRUE AND is_bank IS FALSE)
                    OR (is_cash IS FALSE AND is_bank IS TRUE)
                )
);