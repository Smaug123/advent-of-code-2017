pub mod day_13 {

    pub struct Layer {
        depth: u8,
        range: u8,
    }

    pub fn parse(s: &str) -> Layer {
        let mut chars = s.chars();
        let mut depth = 0u8;
        while let Some(i) = chars.next().unwrap().to_digit(10) {
            depth = depth * 10 + i as u8;
        }
        match chars.next() {
            Some(' ') => {}
            x => {
                panic!("Expected space, got: {:?}", x);
            }
        }
        let mut range = 0u8;
        for s in chars {
            range = range * 10 + s.to_digit(10).unwrap() as u8;
        }
        Layer { depth, range }
    }

    pub fn input() -> Vec<Layer> {
        let input = include_str!("../input.txt");
        input.lines().map(parse).collect::<Vec<_>>()
    }

    pub(crate) fn pos_at_time(range: u8, t: u8) -> u8 {
        // 0, 1, 2, .. , depth-2, depth-1, depth-2, ..., 2, 1, 0, 1, ...
        // Cycle length of 0, 1, .., 2, 1 is 2*(depth - 1)
        if range == 1 {
            return 0;
        }
        let t = t % (2 * range - 2);
        if t < range {
            t
        } else {
            2 * range - 2 - t
        }
    }

    pub fn part_1(layers: &[Layer]) -> u32 {
        layers
            .iter()
            .filter_map(|i| {
                let security_pos = pos_at_time(i.range, i.depth);
                if security_pos == 0 {
                    Some(i.range as u32 * i.depth as u32)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn part_2(layers: &[Layer]) -> u32 {
        // Collection of constraints: (a, b) means we can't be a mod b.
        let targets = {
            let mut targets: Vec<(u32, u32)> = layers
                .iter()
                .map(|i| {
                    let d = 2 * i.range - 2;
                    let mut top = d;
                    while top < i.depth {
                        top += d;
                    }
                    (((top - i.depth) % d) as u32, d as u32)
                })
                .collect();
            targets.sort_unstable_by_key(|(_, j)| *j);
            targets
        };
        // It is possible to do much better here by finding the constraints mod primes,
        // and using the Chinese remainder theorem.
        // Life's too short, when this takes 0.04secs anyway.
        (0..)
            .find(|i| {
                targets
                    .iter()
                    .find(|(target, modulus)| i % modulus == *target)
                    .is_none()
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::day_13::*;

    #[test]
    fn test_pos_at_time() {
        for (time, count) in [0, 1, 2, 3, 2, 1, 0, 1, 2, 3, 2, 1, 0, 1, 2, 3]
            .iter()
            .enumerate()
        {
            assert_eq!(pos_at_time(4, time as u8), *count);
        }
    }

    #[test]
    fn part1_known() {
        let data: Vec<Layer> = vec!["0: 3", "1: 2", "4: 4", "6: 4"]
            .iter()
            .map(|i| parse(i))
            .collect();
        assert_eq!(part_1(&data), 24);
    }

    #[test]
    fn part2_known() {
        let data: Vec<Layer> = vec!["0: 3", "1: 2", "4: 4", "6: 4"]
            .iter()
            .map(|i| parse(i))
            .collect();
        assert_eq!(part_2(&data), 10);
    }

    #[test]
    fn test_day_13() {
        let input = input();
        assert_eq!(part_1(&input), 1904);
        assert_eq!(part_2(&input), 3833504);
    }
}
