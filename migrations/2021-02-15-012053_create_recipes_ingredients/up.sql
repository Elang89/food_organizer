-- Your SQL goes here
CREATE TABLE recipes_ingredients (
    recipe_id uuid REFERENCES recipes (id),
    ingredient_id uuid REFERENCES ingredients (id),
    PRIMARY KEY (recipe_id, ingredient_id))
