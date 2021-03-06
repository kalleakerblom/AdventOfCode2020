use std::ops::RangeInclusive;
type Ranges = Vec<RangeInclusive<usize>>;
struct Rule(String, Ranges);
type Ticket = Vec<usize>;

fn parse(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut sections = input.split("\r\n\r\n");
    let rules_sec = sections.next().unwrap();
    let rules = rules_sec
        .lines()
        .map(|l| {
            let (field, ranges) = l.split_once(": ").unwrap();
            let ranges = ranges
                .split(" or ")
                .map(|r| {
                    let (s, e) = r
                        .split_once('-')
                        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
                        .unwrap();
                    s..=e
                })
                .collect();
            Rule(field.to_string(), ranges)
        })
        .collect();
    let my_sec = sections.next().unwrap();
    let my_ticket: Ticket = my_sec
        .lines()
        .nth(1)
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap();
    let nearby_sec = sections.next().unwrap();
    let nearby_tickets: Vec<Ticket> = nearby_sec
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    (rules, my_ticket, nearby_tickets)
}

fn do_part1(rules: &[Rule], nearby_tickets: &[Ticket]) -> usize {
    nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|val| {
                    let valid = rules.iter().any(|Rule(_, range)| {
                        range.iter().any(|r| r.contains(val))
                    });
                    if valid {
                        0
                    } else {
                        *val
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn do_part2(
    rules: &[Rule],
    my_ticket: &[usize],
    nearby_tickets: &[Ticket],
) -> usize {
    let in_range = |val, ranges: &[RangeInclusive<usize>]| {
        ranges.iter().any(|r| r.contains(&val))
    };

    let valid_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|v| rules.iter().any(|Rule(_, r)| in_range(*v, r)))
        })
        .collect();

    let mut possible_fields: Vec<Vec<&str>> = (0..rules.len())
        .map(|pos| {
            let vals_at_pos = valid_tickets.iter().map(|ticket| ticket[pos]);
            rules
                .iter()
                .filter(|Rule(_, ranges)| {
                    vals_at_pos.clone().all(|v| in_range(v, ranges))
                })
                .map(|Rule(s, _)| s.as_str())
                .collect()
        })
        .collect();

    let mut solved_fields = Vec::with_capacity(rules.len());
    while solved_fields.len() < rules.len() {
        let (pos, solved_field) = possible_fields
            .iter()
            .enumerate()
            .find(|(_, fields)| fields.len() == 1)
            .map(|(i, f)| (i, f[0]))
            .unwrap();
        possible_fields
            .iter_mut()
            .for_each(|fields| fields.retain(|f| f != &solved_field));
        solved_fields.push((pos, solved_field));
    }
    solved_fields
        .iter()
        .filter_map(|(pos, field)| {
            if field.contains("departure") {
                Some(my_ticket[*pos])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{do_part1, do_part2, parse};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example16").unwrap();
        let (rules, _, nearby) = parse(&input);
        let ans = do_part1(&rules, &nearby);
        assert_eq!(ans, 71);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day16").unwrap();
        let (rules, _, nearby) = parse(&input);
        let ans = do_part1(&rules, &nearby);
        assert_eq!(ans, 21978);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day16").unwrap();
        let (rules, my_ticket, nearby) = parse(&input);
        let ans = do_part2(&rules, &my_ticket, &nearby);
        assert_eq!(ans, 1053686852011);
    }
}
