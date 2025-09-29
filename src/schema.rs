// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        sale_price -> Float8,
        cost -> Float8,
        stock -> Float8,
        description -> Nullable<Text>,
    }
}
