import uuid
import string
import random
import subprocess
import os
import pprint


INSERT_RECIPE = "INSERT INTO recipes (id, name, description) VALUES "
INSERT_INGREDIENT = "INSERT INTO ingredients (id, name, description) VALUES "
INSERT_RECIPE_INGREDIENT = "INSERT INTO recipes_ingredients (recipe_id, ingredient_id) VALUES "

RECIPE_TEMPLATE = string.Template("('$id', '$name', '$description')")
INGREDIENT_TEMPLATE = string.Template("('$id', '$name', '$description')")
RECIPE_INGREDIENT_TEMPLATE = string.Template("('$recipe_id', '$ingredient_id')")
PGPASSWORD = "password"

ITERATIONS = 200

def generate_random_string(length=5):
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for i in range(length))


def main():
    recipes_associations = []
    recipes = []
    recipe_ingredients_list= []
    ingredients = []
    ingredient_list = []
    os.environ["PGPASSWORD"] = PGPASSWORD

    for _ in range(0, ITERATIONS):
        i_id = uuid.uuid4()
        i_name = generate_random_string()
        i_description = generate_random_string()

        ingredients.append(i_id)

        ingredient_list.append(INGREDIENT_TEMPLATE.substitute(id=i_id, name=i_name, description=i_description))

    ingredients_string = ",".join(ingredient_list)
    ingredient = "".join([INSERT_INGREDIENT, ingredients_string])

    for _ in range (0, ITERATIONS): 
        r_id = uuid.uuid4()
        r_name = generate_random_string()
        r_description = generate_random_string()

        ingredient1 = random.choice(ingredients)
        ingredient2 = random.choice(ingredients)

        recipes_associations.append({"recipe": (r_id, r_name, r_description), "ingredient1": ingredient1, "ingredient2": ingredient2})


    for recipe in recipes_associations:
        r_id, r_name, r_description = recipe.get("recipe")
        i_1 = recipe.get("ingredient1")
        i_2 = recipe.get("ingredient2")

        recipe_insertion = RECIPE_TEMPLATE.substitute(id=r_id, name=r_name, description=r_description)
        recipes_ingredients_insertion1 = RECIPE_INGREDIENT_TEMPLATE.substitute(recipe_id=r_id, ingredient_id=i_1)
        recipes_ingredients_insertion2 = RECIPE_INGREDIENT_TEMPLATE.substitute(recipe_id=r_id, ingredient_id=i_2)

        recipes.append(recipe_insertion)
        recipe_ingredients_list.append(recipes_ingredients_insertion1)
        recipe_ingredients_list.append(recipes_ingredients_insertion2)

    recipe_string = ",".join(recipes)
    recipe_ingredient_string = ",".join(recipe_ingredients_list)

    recipe = "".join([INSERT_RECIPE, recipe_string])
    recipe_ingredient = "".join([INSERT_RECIPE_INGREDIENT, recipe_ingredient_string])

    subprocess.run(["psql", "-d", "food", "-h", "localhost", "-U", "root", "-c", ingredient])
    subprocess.run(["psql", "-d", "food", "-h", "localhost", "-U", "root", "-c", recipe])
    subprocess.run(["psql", "-d", "food", "-h", "localhost", "-U", "root", "-c", recipe_ingredient])


if __name__ == "__main__":
    main()