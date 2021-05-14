pub mod day_6 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn input() -> Vec<u32> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split_whitespace()
            .map(|i| {
                i.parse()
                    .unwrap_or_else(|_| panic!("{} wasn't a valid u32", i))
            })
            .collect::<Vec<u32>>()
    }

    pub fn part_1(v: &mut [u32]) -> u32 {
        let len = v.len() as u32;
        let mut seen: HashSet<Vec<u32>> = HashSet::new();
        let mut count = 0;
        let (mut max_pos, mut max) = v
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|(_, x)| *x)
            .unwrap();
        while seen.insert(v.to_vec()) {
            let extras = max % len;
            let all = max / len;
            v[max_pos] = 0;
            for i in 0..(extras as usize) {
                v[(max_pos + i + 1) % v.len()] += 1;
            }
            max = 0;
            for (i, item) in v.iter_mut().enumerate() {
                *item += all;
                if *item > max {
                    max = *item;
                    max_pos = i;
                }
            }
            count += 1;
        }
        count
    }

    pub fn part_2(v: &mut [u32]) -> u32 {
        let len = v.len() as u32;
        let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
        let mut count = 0;
        let (mut max_pos, mut max) = v
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|(_, x)| *x)
            .unwrap();
        loop {
            if let Some(existing) = seen.insert(v.to_vec(), count) {
                return count - existing;
            }
            let extras = max % len;
            let all = max / len;
            v[max_pos] = 0;
            for i in 0..(extras as usize) {
                v[(max_pos + i + 1) % v.len()] += 1;
            }
            max = 0;
            for (i, item) in v.iter_mut().enumerate() {
                *item += all;
                if *item > max {
                    max = *item;
                    max_pos = i;
                }
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day_6::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&mut vec![0, 2, 7, 0]), 5);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&mut vec![0, 2, 7, 0]), 4);
    }

    #[test]
    fn test_day_6() {
        let input = input();
        let answer = part_1(&mut input.clone());
        assert_eq!(answer, 4074);
        let mut input = input;
        let answer = part_2(&mut input);
        assert_eq!(answer, 2793);
    }
}
