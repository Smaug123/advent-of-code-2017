pub mod day_22 {
    use std::collections::HashMap;

    enum State {
        Clean,
        Weakened,
        Flagged,
        Infected,
    }

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    impl Direction {
        fn turn_right(d: &Direction) -> Direction {
            match *d {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            }
        }

        fn turn_left(d: &Direction) -> Direction {
            match *d {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
            }
        }

        fn reverse(d: &Direction) -> Direction {
            match *d {
                Direction::Left => Direction::Right,
                Direction::Up => Direction::Down,
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Up,
            }
        }
    }

    pub struct Board {
        squares: HashMap<(i32, i32), State>,
        position: (i32, i32),
        direction: Direction,
    }

    fn move_in_dir(row_col: (i32, i32), direction: &Direction) -> (i32, i32) {
        let (row, col) = row_col;
        match *direction {
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
        }
    }

    impl Board {
        pub(crate) fn parse(s: &str) -> Board {
            let mut output = HashMap::new();
            let mut num_cols = 0;
            let mut num_rows = 0;
            for l in s.lines() {
                let mut col = 0;
                for ch in l.trim().chars() {
                    output.insert(
                        (num_rows, col),
                        match ch {
                            '#' => State::Infected,
                            '.' => State::Clean,
                            c => {
                                panic!("Unexpected char: {}", c);
                            }
                        },
                    );
                    col += 1;
                }
                if num_rows == 0 {
                    num_cols = col;
                }
                num_rows += 1;
            }

            Board {
                squares: output,
                position: (num_rows / 2, num_cols / 2),
                direction: Direction::Up,
            }
        }
    }

    pub fn input() -> Board {
        let input = include_str!("../input.txt");
        Board::parse(&input)
    }

    // Returns whether we caused an infection.
    fn move_once(board: &mut Board) -> bool {
        let mut to_ret = false;
        let entry = board.squares.entry(board.position).or_insert(State::Clean);
        match *entry {
            State::Infected => {
                board.direction = Direction::turn_right(&board.direction);
                *entry = State::Clean;
            }
            State::Clean => {
                board.direction = Direction::turn_left(&board.direction);
                *entry = State::Infected;
                to_ret = true;
            }
            _ => {
                panic!("Unexpected state");
            }
        }
        board.position = move_in_dir(board.position, &board.direction);
        to_ret
    }

    pub fn part_1(mut board: Board, max: u32) -> usize {
        let mut count = 0;
        for _ in 0..max {
            if move_once(&mut board) {
                count += 1;
            }
        }
        count
    }

    // Returns whether we caused an infection.
    fn move_once_2(board: &mut Board) -> bool {
        let mut to_ret = false;
        let entry = board.squares.entry(board.position).or_insert(State::Clean);
        match *entry {
            State::Infected => {
                board.direction = Direction::turn_right(&board.direction);
                *entry = State::Flagged;
            }
            State::Clean => {
                board.direction = Direction::turn_left(&board.direction);
                *entry = State::Weakened;
            }
            State::Flagged => {
                board.direction = Direction::reverse(&board.direction);
                *entry = State::Clean;
            }
            State::Weakened => {
                *entry = State::Infected;
                to_ret = true;
            }
        }
        board.position = move_in_dir(board.position, &board.direction);
        to_ret
    }

    pub fn part_2(mut board: Board, max: u32) -> usize {
        let mut count = 0;
        for _ in 0..max {
            if move_once_2(&mut board) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::day_22::*;

    #[test]
    fn part1_known() {
        let input = Board::parse("..#\n#..\n...");
        assert_eq!(part_1(input, 70), 41);
        let input = Board::parse("..#\n#..\n...");
        assert_eq!(part_1(input, 10000), 5587);
    }

    #[test]
    fn part2_known() {
        let input = Board::parse("..#\n#..\n...");
        assert_eq!(part_2(input, 100), 26);
        let input = Board::parse("..#\n#..\n...");
        assert_eq!(part_2(input, 10000000), 2511944);
    }

    #[test]
    fn test_day_22() {
        let board = input();
        assert_eq!(part_1(board, 10000), 5447);
        let input = input();
        assert_eq!(part_2(input, 10000000), 2511705);
    }
}
