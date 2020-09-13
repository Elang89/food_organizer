table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
    }
}
