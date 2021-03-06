pub mod day_1 {

    pub fn input() -> Vec<u32> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .chars()
            .map(|l| {
                l.to_digit(10)
                    .unwrap_or_else(|| panic!("{} wasn't a valid u32", l))
            })
            .collect::<Vec<u32>>()
    }

    pub fn part_1(numbers: &[u32]) -> u32 {
        let mut sum = 0;
        let mut previous = numbers[0];
        let len = numbers.len();
        for &item in numbers.iter().skip(1) {
            if item == previous {
                sum += previous;
            }
            previous = item;
        }
        if len <= 1 {
            // Start = end, so no need to compare last with first
            return sum;
        }

        if previous == numbers[0] {
            sum += previous;
        }

        sum
    }

    pub fn part_2(numbers: &[u32]) -> u32 {
        let mut sum = 0;
        let len = numbers.len();
        for i in 0..len / 2 {
            if numbers[i] == numbers[len / 2 + i] {
                sum += 2 * numbers[i];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::day_1::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&[1, 1, 2, 2]), 3);
        assert_eq!(part_1(&[1, 1, 1, 1]), 4);
        assert_eq!(part_1(&[1, 2, 3, 4]), 0);
        assert_eq!(part_1(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&[1, 2, 1, 2]), 6);
        assert_eq!(part_2(&[1, 2, 2, 1]), 0);
        assert_eq!(part_2(&[1, 2, 3, 4, 2, 5]), 4);
        assert_eq!(part_2(&[1, 2, 3, 1, 2, 3]), 12);
        assert_eq!(part_2(&[1, 2, 1, 3, 1, 4, 1, 5]), 4);
    }

    #[test]
    fn test_day_1() {
        let input = input();
        assert_eq!(part_1(&input), 1223);
        assert_eq!(part_2(&input), 1284);
    }
}
