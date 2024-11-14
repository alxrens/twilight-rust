// @generated automatically by Diesel CLI.

diesel::table! {
    magi_memories (id) {
        id -> Varchar,
        user_id -> Varchar,
        memory -> Varchar,
    }
}
