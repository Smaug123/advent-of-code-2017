pub mod day_4 {

    use std::collections::HashSet;
    use std::hash::Hash;

    pub fn input() -> Vec<Vec<&'static str>> {
        let input = include_str!("../input.txt");
        input
            .lines()
            .map(|l| l.split_whitespace().collect())
            .collect()
    }

    fn contains_duplicate<I, X>(i: &mut I) -> bool
    where
        I: Iterator<Item = X>,
        X: Eq + Hash,
    {
        let mut so_far = HashSet::new();
        for elt in i {
            if !so_far.insert(elt) {
                return true;
            }
        }
        false
    }

    pub fn part_1(input: &[Vec<&str>]) -> usize {
        input
            .iter()
            .filter(|words| !contains_duplicate(&mut words.iter()))
            .count()
    }

    pub fn part_2(input: &[Vec<&str>]) -> usize {
        input
            .iter()
            .filter(|words| {
                !contains_duplicate(&mut words.iter().map(|&w| {
                    let mut w = w.chars().collect::<Vec<char>>();
                    w.sort_unstable();
                    w
                }))
            })
            .count()
    }

}

#[cfg(test)]
mod tests {
    use super::day_4::*;

    #[test]
    fn part1_known() {
        assert_eq!(
            part_1(&vec![
                vec!["aa", "bb", "cc", "dd", "ee"],
                vec!["aa", "bb", "cc", "dd", "aa"],
                vec!["aa", "bb", "cc", "dd", "aaa"]
            ]),
            2
        );
    }

    #[test]
    fn part2_known() {
        assert_eq!(
            part_2(&vec![
                vec!["abcde", "fghij"],
                vec!["abcde", "xyz", "ecdab"],
                vec!["a", "ab", "abc", "abd", "abf", "abj"],
                vec!["iiii", "oiii", "ooii", "oooi", "oooo"],
                vec!["oiii", "ioii", "iioi", "iiio"]
            ]),
            3
        );
    }

    #[test]
    fn test_day_4() {
        let input = input();
        assert_eq!(part_1(&input), 325);
        assert_eq!(part_2(&input), 119);
    }
}
