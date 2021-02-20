table! {
    ingredients (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    recipes_ingredients (recipe_id, ingredient_id) {
        recipe_id -> Uuid,
        ingredient_id -> Uuid,
    }
}

joinable!(recipes_ingredients -> ingredients (ingredient_id));
joinable!(recipes_ingredients -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
    recipes_ingredients,
);
