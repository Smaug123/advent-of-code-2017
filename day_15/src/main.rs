fn input() -> (u32, u32) {
    let mut inputs = include_str!("../input.txt")
        .lines()
        .map(|l| l.trim().split_whitespace().last().unwrap().parse().unwrap());

    let a = inputs.next().unwrap();
    let b = inputs.next().unwrap();
    match inputs.next() {
        Some(i) => panic!("Expected no more lines, got {}", i),
        None => (a, b),
    }
}

struct Generator {
    item: u32,
    modulus: u64,
}

impl Iterator for Generator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_item = ((self.item as u64) * self.modulus % 2147483647) as u32;
        self.item = new_item;
        Some(new_item)
    }
}

fn part_1(a: u32, b: u32) -> usize {
    let gen_a = Generator {
        item: a,
        modulus: 16807,
    };
    let gen_b = Generator {
        item: b,
        modulus: 48271,
    };

    Iterator::zip(gen_a, gen_b)
        .take(40000000)
        .filter(|(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

fn part_2(a: u32, b: u32) -> usize {
    let gen_a = Generator {
        item: a,
        modulus: 16807,
    };
    let gen_b = Generator {
        item: b,
        modulus: 48271,
    };

    Iterator::zip(gen_a.filter(|i| i % 4 == 0), gen_b.filter(|i| i % 8 == 0))
        .take(5000000)
        .filter(|(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

fn main() {
    let (a, b) = input();
    println!("part 1 => {}", part_1(a, b));
    println!("part 2 => {}", part_2(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(65, 8921), 588);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(65, 8921), 309);
    }

    #[test]
    fn test_day_15() {
        let (a, b) = input();
        assert_eq!(part_1(a, b), 638);
        assert_eq!(part_2(a, b), 343);
    }
}
