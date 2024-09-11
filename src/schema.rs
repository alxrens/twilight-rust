// @generated automatically by Diesel CLI.

diesel::table! {
    anicard (id) {
        id -> Varchar,
        character_name -> Varchar,
        favorites -> Int4,
        anime -> Varchar,
        img -> Varchar,
    }
}

diesel::table! {
    anicard_user (id) {
        id -> Varchar,
        currency -> Nullable<Int4>,
    }
}

diesel::table! {
    inventory (id) {
        id -> Varchar,
        owner_id -> Varchar,
    }
}

diesel::table! {
    inventory_anicard (id) {
        id -> Varchar,
        anicard_id -> Int4,
        inventory_id -> Int4,
        status_payment -> Nullable<Varchar>,
        left_amount -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anicard,
    anicard_user,
    inventory,
    inventory_anicard,
);
