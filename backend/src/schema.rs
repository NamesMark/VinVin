// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        wine_name -> Varchar,
        wine_year -> Nullable<Int4>,
        region -> Nullable<Varchar>,
        look -> Nullable<Text>,
        nose -> Nullable<Text>,
        palate -> Nullable<Text>,
        tail -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        profile_pic -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        bio -> Nullable<Text>,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
