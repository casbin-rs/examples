table! {
    todos (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        finished -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
