//! https://adventofcode.com/2020/day/21

use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/2020/day21.txt");

    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
    println!("Two: {}", solve2(&puzzle));
}

fn parse(input: &str) -> Puzzle {
    let mut foods = Vec::new();

    for line in input.lines() {
        let mut parts = line.strip_suffix(")").unwrap().split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let allergens = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        foods.push(Food {
            ingredients,
            allergens,
        });
    }

    Puzzle { foods }
}

fn solve(puzzle: &Puzzle) -> usize {
    let mut assignment: HashMap<&String, HashSet<&String>> = HashMap::new();
    for food in &puzzle.foods {
        for allergen in &food.allergens {
            if let Some(ing) = assignment.get_mut(allergen) {
                ing.retain(|k| food.ingredients.contains(k));
            } else {
                assignment.insert(allergen, food.ingredients.iter().collect());
            }
        }
    }

    let all_allergic_foods: HashSet<_> = assignment.values().flat_map(|v| v).collect();
    puzzle
        .foods
        .iter()
        .flat_map(|f| &f.ingredients)
        .filter(|i| !all_allergic_foods.contains(&i))
        .count()
}

fn solve2(puzzle: &Puzzle) -> String {
    let mut allergen_to_ing: HashMap<&String, HashSet<&String>> = HashMap::new();
    for food in &puzzle.foods {
        for allergen in &food.allergens {
            if let Some(ing) = allergen_to_ing.get_mut(allergen) {
                ing.retain(|k| food.ingredients.contains(k));
            } else {
                allergen_to_ing.insert(allergen, food.ingredients.iter().collect());
            }
        }
    }

    let mut assignment: BTreeMap<&String, &String> = BTreeMap::new();
    loop {
        let allergen_with_single = allergen_to_ing
            .iter()
            .find(|(_k, v)| v.len() == 1)
            .map(|(k, _v)| k.clone());
        if let Some(allergen) = allergen_with_single {
            if let Some(single_ingredient) = allergen_to_ing.remove(allergen) {
                let ingredient = single_ingredient.into_iter().next().unwrap();
                assignment.insert(allergen, ingredient);
                for (_a, ingredients) in &mut allergen_to_ing {
                    ingredients.remove(ingredient);
                }
            }
        } else {
            break;
        }
    }

    assignment.values().join(",")
}

#[derive(Clone, Debug)]
struct Puzzle {
    foods: Vec<Food>,
}

#[derive(Clone, Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        let puzzle = parse(input);

        assert_eq!(solve(&puzzle), 5);
        assert_eq!(solve2(&puzzle), "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
