table! {
    products (id) {
        id -> Int4,
        name -> Text,
        price -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
