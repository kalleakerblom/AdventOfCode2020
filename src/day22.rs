use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

fn parse_decks(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let (p1, p2) = input.split_once("\r\n\r\n").unwrap();
    let deck1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    let deck2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    (deck1, deck2)
}

fn calc_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as usize * card)
        .sum()
}

#[derive(Debug)]
enum Winner {
    P1,
    P2,
}

fn crab_combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (Winner, usize) {
    loop {
        if deck1.is_empty() {
            return (Winner::P2, calc_score(&deck2));
        }
        if deck2.is_empty() {
            return (Winner::P1, calc_score(&deck1));
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
}

fn recursive_combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (Winner, usize) {
    let mut history: HashSet<u64> = HashSet::new();
    // game loop
    loop {
        let mut hasher = DefaultHasher::new();
        deck1.hash(&mut hasher);
        deck2.hash(&mut hasher);
        if !history.insert(hasher.finish()) {
            return (Winner::P1, calc_score(&deck1));
        }
        if deck1.is_empty() {
            return (Winner::P2, calc_score(&deck2));
        }
        if deck2.is_empty() {
            return (Winner::P1, calc_score(&deck1));
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 <= deck1.len() && card2 <= deck2.len() {
            // sub-game!
            let subdeck1 = deck1.iter().take(card1).cloned().collect();
            let subdeck2 = deck2.iter().take(card2).cloned().collect();
            match recursive_combat(subdeck1, subdeck2) {
                (Winner::P1, _) => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                (Winner::P2, _) => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day22::parse_decks;
    use crate::day22::{crab_combat, recursive_combat};
    #[test]
    fn example1() {
        let input = fs::read_to_string("input/example22").unwrap();
        let (deck1, deck2) = parse_decks(&input);
        let ans = crab_combat(deck1, deck2);
        dbg!(ans);
    }

    #[test]
    fn example2() {
        let input = fs::read_to_string("input/example22").unwrap();
        let (deck1, deck2) = parse_decks(&input);
        let ans = recursive_combat(deck1, deck2);
        dbg!(ans.1);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day22").unwrap();
        let (deck1, deck2) = parse_decks(&input);
        let ans = crab_combat(deck1, deck2);
        dbg!(ans);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day22").unwrap();
        let (deck1, deck2) = parse_decks(&input);
        let ans = recursive_combat(deck1, deck2);
        assert_eq!(ans.1, 32534);
    }
}
