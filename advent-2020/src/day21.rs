use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (ingredients_str, allergens_str) = s
            .split_once(" (contains ")
            .map(|(i, a)| (i, a.trim_end_matches(')')))
            .unwrap_or((s, ""));

        Ok(Food {
            ingredients: ingredients_str
                .split_whitespace()
                .map(String::from)
                .collect(),
            allergens: if allergens_str.is_empty() {
                vec![]
            } else {
                allergens_str.split(", ").map(String::from).collect()
            },
        })
    }
}

#[aoc_generator(day21)]
fn generator(input: &str) -> Result<Vec<Food>> {
    input.lines().map(str::parse).collect()
}

fn find_allergen_possibilities(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    foods.iter().fold(HashMap::new(), |mut map, food| {
        let ingredient_set: HashSet<_> = food.ingredients.iter().cloned().collect();

        for allergen in &food.allergens {
            map.entry(allergen.clone())
                .and_modify(|possibilities| {
                    *possibilities = possibilities
                        .intersection(&ingredient_set)
                        .cloned()
                        .collect();
                })
                .or_insert_with(|| ingredient_set.clone());
        }
        map
    })
}

#[aoc(day21, part1)]
fn part1(foods: &[Food]) -> Result<usize> {
    let allergen_possibilities = find_allergen_possibilities(foods);

    let ingredients_with_allergens: HashSet<_> =
        allergen_possibilities.values().flatten().collect();

    Ok(foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .filter(|ingredient| !ingredients_with_allergens.contains(ingredient))
        .count())
}

#[aoc(day21, part2)]
fn part2(foods: &[Food]) -> Result<String> {
    let allergen_possibilities = find_allergen_possibilities(foods);
    let allergen_to_ingredient = resolve_allergens(&allergen_possibilities);

    let mut sorted_allergens: Vec<_> = allergen_to_ingredient.keys().cloned().collect();
    sorted_allergens.sort();

    Ok(sorted_allergens
        .into_iter()
        .map(|allergen| allergen_to_ingredient[&allergen].clone())
        .collect::<Vec<_>>()
        .join(","))
}

fn resolve_allergens(possibilities: &HashMap<String, HashSet<String>>) -> HashMap<String, String> {
    let mut resolved = HashMap::new();

    while resolved.len() < possibilities.len() {
        let newly_resolved: Vec<_> = possibilities
            .iter()
            .filter_map(|(allergen, ingredients)| {
                let remaining: HashSet<_> = ingredients
                    .iter()
                    .filter(|i| !resolved.values().any(|v: &String| v == *i))
                    .cloned()
                    .collect();

                (remaining.len() == 1)
                    .then(|| (allergen.clone(), remaining.into_iter().next().unwrap()))
            })
            .collect();

        for (allergen, ingredient) in newly_resolved {
            resolved.insert(allergen, ingredient);
        }
    }

    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let foods = generator(input).unwrap();
        assert_eq!(part1(&foods).unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let foods = generator(input).unwrap();
        assert_eq!(part2(&foods).unwrap(), "mxmxvkd,sqjhc,fvjkl");
    }
}
