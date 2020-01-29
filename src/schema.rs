table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        published -> Bool,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, users,);
