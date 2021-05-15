pub mod day_12 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::hash::Hash;

    pub struct Node {
        number: u32,
        friends: Vec<u32>,
    }

    pub fn parse(s: &str) -> Node {
        let mut iter = s.split_whitespace();
        let number: u32 = iter.next().unwrap().parse().unwrap();
        match iter.next().unwrap() {
            "<->" => {}
            s => panic!("Bad! {}", s),
        }

        let friends = iter
            .map(|s| {
                let mut ans = 0;
                for d in s.chars() {
                    match d.to_digit(10) {
                        None => break,
                        Some(d) => ans = ans * 10 + d,
                    }
                }
                ans
            })
            .collect::<Vec<_>>();

        Node { number, friends }
    }

    pub fn input() -> Vec<Node> {
        let input = include_str!("../input.txt");
        input.lines().map(|l| parse(l)).collect::<Vec<_>>()
    }

    fn connected_component<T>(relations: &HashMap<T, &[T]>, component: T) -> HashSet<T>
    where
        T: Eq + Hash + Clone,
    {
        let mut stack: Vec<_> = relations.get(&component).unwrap().to_vec();
        let mut connected: HashSet<_> = HashSet::new();
        connected.insert(component);

        while let Some(explore) = stack.pop() {
            if connected.insert(explore.clone()) {
                let relatives = relations.get(&explore).unwrap().iter();
                stack.extend(relatives.filter(|i| !connected.contains(&i)).cloned());
            }
        }

        connected
    }

    pub fn part_1(input: &[Node]) -> usize {
        let relations: HashMap<u32, &[u32]> = input
            .iter()
            .map(|n| (n.number, n.friends.as_slice()))
            .collect();
        connected_component(&relations, 0).len()
    }

    pub fn part_2(input: &[Node]) -> u32 {
        let relations: HashMap<u32, &[u32]> = input
            .iter()
            .map(|n| (n.number, n.friends.as_slice()))
            .collect();

        let mut count = 0;
        let mut components_found = HashSet::new();
        for node in input {
            if !components_found.contains(&node.number) {
                let component = connected_component(&relations, node.number);
                components_found.extend(component);
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::day_12::*;

    fn data() -> Vec<Node> {
        vec![
            "0 <-> 2",
            "1 <-> 1",
            "2 <-> 0, 3, 4",
            "3 <-> 2, 4",
            "4 <-> 2, 3, 6",
            "5 <-> 6",
            "6 <-> 4, 5",
        ]
        .iter()
        .map(|s| parse(s))
        .collect()
    }

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&data()), 6);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&data()), 2);
    }

    #[test]
    fn test_day_12() {
        let input = input();
        assert_eq!(part_1(&input), 378);
        assert_eq!(part_2(&input), 204);
    }
}
