table! {
    casbin_rules (id) {
        id -> Int4,
        ptype -> Varchar,
        v0 -> Varchar,
        v1 -> Varchar,
        v2 -> Varchar,
        v3 -> Varchar,
        v4 -> Varchar,
        v5 -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Int4,
        is_deleted -> Bool,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        login_session -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    casbin_rules,
    posts,
    users,
);
