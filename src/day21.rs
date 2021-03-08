use std::collections::{HashMap, HashSet};
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse(input: &str) -> Vec<Food> {
    //mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    input
        .lines()
        .map(|l| {
            let (ingredients, allergens) = l.split_once(" (contains ").unwrap();
            let ingredients =
                ingredients.split_whitespace().map(str::to_string).collect();
            let allergens = allergens
                .trim_end_matches(')')
                .split(", ")
                .map(str::to_string)
                .collect();
            Food { ingredients, allergens }
        })
        .collect()
}

fn map_allergens(foods: &[Food]) -> HashMap<String, Vec<usize>> {
    let mut res: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, f) in foods.iter().enumerate() {
        for a in &f.allergens {
            res.entry(a.clone()).or_default().push(i);
        }
    }
    res
}
fn find_allergy_solve(
    a_map: HashMap<String, Vec<usize>>,
    foods: &[Food],
) -> HashMap<String, String> {
    let mut solved = HashSet::new();
    let mut solved_map = HashMap::new();
    'outer: loop {
        for (allergy, food_idxs) in &a_map {
            let shared_ingredients = food_idxs
                .iter()
                .map(|i| &foods[*i].ingredients)
                .cloned()
                .reduce(|acc, i| &acc & &i)
                .unwrap();
            let shared_ingredients = &shared_ingredients - &solved;
            if shared_ingredients.len() == 1 {
                let solved_ing = shared_ingredients.iter().next().unwrap();
                solved.insert(solved_ing.clone());
                solved_map.insert(solved_ing.clone(), allergy.clone());
                continue 'outer;
            }
        }
        break;
    }
    solved_map
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{find_allergy_solve, map_allergens, parse};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example21").unwrap();
        let _foods = parse(&input);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day21").unwrap();
        let foods = parse(&input);
        let t = map_allergens(&foods);
        let solved = find_allergy_solve(t, &foods);
        let ans = foods
            .iter()
            .flat_map(|f| f.ingredients.iter())
            .filter(|i| !solved.contains_key(*i))
            .count();
        assert_eq!(ans, 2423);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day21").unwrap();
        let foods = parse(&input);
        let t = map_allergens(&foods);
        let solved = find_allergy_solve(t, &foods);
        let mut dangerous: Vec<_> = solved.iter().collect();
        dangerous.sort_by_key(|&(_, allergy)| allergy);
        let ans: Vec<&str> =
            dangerous.into_iter().map(|(i, _)| i.as_str()).collect();
        let ans = ans.join(",");
        assert_eq!(ans, "jzzjz,bxkrd,pllzxb,gjddl,xfqnss,dzkb,vspv,dxvsp");
    }
}
