CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    customer_id INT NOT NULL REFERENCES contacts (id),
    amount_total DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_details (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    product_id INT NOT NULL REFERENCES products (id),
    quantity DOUBLE PRECISION NOT NULL,
    price_unit DOUBLE PRECISION NOT NULL,
    amount_total DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE payment_orders (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    payment_method_id INT NOT NULL REFERENCES payment_methods (id),
    amount_paid DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_order_details_updated_at BEFORE UPDATE ON order_details FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_payment_orders_updated_at BEFORE UPDATE ON payment_orders FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();