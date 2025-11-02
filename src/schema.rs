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
        stock -> Float8,
        description -> Nullable<Text>,
        category_id -> Int4,
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

diesel::joinable!(products -> product_categories (category_id));
diesel::joinable!(users -> contacts (contact_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    payment_methods,
    product_categories,
    products,
    users,
);
