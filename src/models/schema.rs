// @generated automatically by Diesel CLI.

diesel::table! {
    cocktails (id) {
        id -> Int4,
        name -> Varchar,
        ingredients -> Text,
        recipe -> Text,
    }
}

diesel::table! {
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cocktails,
    todos,
);
