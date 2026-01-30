-- Units of Measure
CREATE TABLE uoms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE, -- e.g. 'pcs', 'kg'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Physical Warehouses
CREATE TABLE warehouses (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE,
    address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Specific Locations (Stock, Shelf, Receiving, etc)
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    warehouse_id INT NOT NULL REFERENCES warehouses (id) ON DELETE CASCADE,
    is_scrap BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Stock Quants (Quantity per Location)
CREATE TABLE stock_quants (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    location_id INT NOT NULL REFERENCES locations (id) ON DELETE CASCADE,
    quantity DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (product_id, location_id)
);

-- Update Products table
ALTER TABLE products ADD COLUMN sku VARCHAR(255) UNIQUE;

ALTER TABLE products ADD COLUMN uom_id INT REFERENCES uoms (id);

-- We remove the simple 'stock' column from products as it is now managed via locations
-- But first, we might want to migrate existing stock to a default location.
-- For simplicity in this demo, we'll just drop it.
-- IN A PRODUCTION APP: You would migrate this data!
ALTER TABLE products DROP COLUMN stock;

-- Triggers for updated_at
CREATE TRIGGER update_uoms_updated_at BEFORE UPDATE ON uoms FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_warehouses_updated_at BEFORE UPDATE ON warehouses FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_locations_updated_at BEFORE UPDATE ON locations FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_stock_quants_updated_at BEFORE UPDATE ON stock_quants FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();