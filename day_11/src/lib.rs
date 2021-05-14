pub mod day_11 {
    pub enum Direction {
        South,
        North,
        NorthEast,
        NorthWest,
        SouthEast,
        SouthWest,
    }

    fn parse(s: &str) -> Direction {
        let mut chars = s.chars();
        let c1 = chars.next().unwrap();
        match chars.next() {
            None => match c1 {
                's' => Direction::South,
                'n' => Direction::North,
                c1 => panic!("Expected south or north, got: {}", c1),
            },
            Some(c2) => match c1 {
                's' => match c2 {
                    'e' => Direction::SouthEast,
                    'w' => Direction::SouthWest,
                    c2 => panic!("Expected SE or SW, got s{}", c2),
                },
                'n' => match c2 {
                    'e' => Direction::NorthEast,
                    'w' => Direction::NorthWest,
                    c2 => panic!("Expected NE or NW, got n{}", c2),
                },
                c1 => panic!("Expected north or south for first direction, got: {}", c1),
            },
        }
    }

    pub fn input() -> Vec<Direction> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split(',')
            .map(parse)
            .collect::<Vec<Direction>>()
    }

    fn abs(i: i32) -> u32 {
        if i >= 0 {
            i as u32
        } else {
            -i as u32
        }
    }

    fn steps_to(x: i32, y: i32) -> u32 {
        let x = abs(x);
        let y = abs(y);
        y + if 2 * y < x { (x - y) / 2 } else { 0 }
    }

    pub fn part_1(steps: &[Direction]) -> u32 {
        let (final_x, final_y) = steps.iter().fold((0, 0), |(x, y), dir| match *dir {
            Direction::SouthEast => (x - 1, y + 1),
            Direction::NorthEast => (x + 1, y + 1),
            Direction::SouthWest => (x - 1, y - 1),
            Direction::NorthWest => (x + 1, y - 1),
            Direction::South => (x - 2, y),
            Direction::North => (x + 2, y),
        });
        steps_to(final_x, final_y)
    }

    pub fn part_2(steps: &[Direction]) -> u32 {
        let (best, _, _) = steps.iter().fold((0, 0, 0), |(best, x, y), dir| {
            let (new_x, new_y) = match *dir {
                Direction::SouthEast => (x - 1, y + 1),
                Direction::NorthEast => (x + 1, y + 1),
                Direction::SouthWest => (x - 1, y - 1),
                Direction::NorthWest => (x + 1, y - 1),
                Direction::South => (x - 2, y),
                Direction::North => (x + 2, y),
            };
            let new_steps = steps_to(new_x, new_y);
            (std::cmp::max(new_steps, best), new_x, new_y)
        });
        best
    }
}

#[cfg(test)]
mod tests {
    use super::day_11::*;

    #[test]
    fn part1_known() {
        assert_eq!(
            part_1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::NorthEast
            ]),
            3
        );
        assert_eq!(
            part_1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::SouthWest,
                Direction::SouthWest
            ]),
            0
        );
        assert_eq!(
            part_1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::South,
                Direction::South
            ]),
            2
        );
        assert_eq!(
            part_1(&[
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::SouthWest
            ]),
            3
        );
    }

    #[test]
    fn test_day_11() {
        let input = input();
        assert_eq!(part_1(&input), 743);
        assert_eq!(part_2(&input), 1493);
    }
}
