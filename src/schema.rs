// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        mobile -> Nullable<Varchar>,
        address -> Nullable<Text>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 100]
        state -> Nullable<Varchar>,
        #[max_length = 20]
        zipcode -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        warehouse_id -> Int4,
        is_scrap -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    order_details (id) {
        id -> Int4,
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Float8,
        price_unit -> Float8,
        amount_total -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        customer_id -> Int4,
        amount_total -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    payment_methods (id) {
        id -> Int4,
        name -> Text,
        code -> Text,
        is_cash -> Nullable<Bool>,
        is_bank -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    payment_orders (id) {
        id -> Int4,
        order_id -> Int4,
        payment_method_id -> Int4,
        amount_paid -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    product_categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        sale_price -> Float8,
        cost -> Float8,
        description -> Nullable<Text>,
        category_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 255]
        sku -> Nullable<Varchar>,
        uom_id -> Nullable<Int4>,
    }
}

diesel::table! {
    stock_quants (id) {
        id -> Int4,
        product_id -> Int4,
        location_id -> Int4,
        quantity -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    uoms (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        contact_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    warehouses (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        address -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(locations -> warehouses (warehouse_id));
diesel::joinable!(order_details -> orders (order_id));
diesel::joinable!(order_details -> products (product_id));
diesel::joinable!(orders -> contacts (customer_id));
diesel::joinable!(payment_orders -> orders (order_id));
diesel::joinable!(payment_orders -> payment_methods (payment_method_id));
diesel::joinable!(products -> product_categories (category_id));
diesel::joinable!(products -> uoms (uom_id));
diesel::joinable!(stock_quants -> locations (location_id));
diesel::joinable!(stock_quants -> products (product_id));
diesel::joinable!(users -> contacts (contact_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    locations,
    order_details,
    orders,
    payment_methods,
    payment_orders,
    product_categories,
    products,
    stock_quants,
    uoms,
    users,
    warehouses,
);
