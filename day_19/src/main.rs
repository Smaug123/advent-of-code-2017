#[derive(Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Vertical,
    Horizontal,
    Cross,
    Char(char),
}

fn parse(c: char) -> Square {
    match c {
        ' ' => Square::Empty,
        '|' => Square::Vertical,
        '-' => Square::Horizontal,
        '+' => Square::Cross,
        c => Square::Char(c),
    }
}

fn input() -> Vec<Vec<Square>> {
    let input = include_str!("../input.txt");
    let mut output = input
        .lines()
        .map(|l| l.chars().map(parse).collect())
        .collect::<Vec<Vec<_>>>();
    let max_len = output.iter().map(|l| l.len()).max().unwrap();
    for row in output.iter_mut() {
        if row.len() < max_len {
            row.extend(std::iter::repeat(Square::Empty).take(max_len - row.len()))
        }
    }
    output
}

fn execute(square: &[Vec<Square>]) -> (String, u32) {
    let mut row = 0;
    let mut col = square[0]
        .iter()
        .enumerate()
        .filter_map(|(i, &j)| if j == Square::Vertical { Some(i) } else { None })
        .next()
        .unwrap();

    let mut answer: Vec<char> = vec![];

    let mut is_vertical = true;
    let mut increasing = true;
    let mut steps = 0;

    loop {
        steps += 1;
        match square[row][col] {
            Square::Empty => {
                return (answer.iter().collect::<String>(), steps - 1);
            }
            Square::Cross => {
                if is_vertical {
                    is_vertical = false;
                    if col == 0 {
                        increasing = true;
                        col = 1;
                    } else if col == square[row].len() - 1 {
                        increasing = false;
                        col = square[row].len() - 2;
                    } else {
                        let left = &square[row][col - 1];
                        let right = &square[row][col + 1];
                        if *left == Square::Empty {
                            increasing = true;
                            col += 1;
                        } else if *right == Square::Empty {
                            increasing = false;
                            col -= 1;
                        } else {
                            panic!("Expected crossroads to move us horizontal.");
                        }
                    }
                } else {
                    is_vertical = true;
                    if row == 0 {
                        increasing = true;
                        row = 1;
                    } else if row == square.len() - 1 {
                        increasing = false;
                        row = square.len() - 2;
                    } else {
                        let top = &square[row - 1][col];
                        let bottom = &square[row + 1][col];
                        if *top == Square::Empty {
                            increasing = true;
                            row += 1;
                        } else if *bottom == Square::Empty {
                            increasing = false;
                            row -= 1;
                        } else {
                            panic!("Expected crossroads to move us vertical.");
                        }
                    }
                }
                continue;
            }
            Square::Char(c) => {
                answer.push(c);
            }
            _ => {}
        }
        if increasing {
            if is_vertical {
                row += 1;
            } else {
                col += 1;
            };
        } else if is_vertical {
            row -= 1;
        } else {
            col -= 1;
        };
    }
}

fn main() {
    let input = input();
    let (part_1, part_2) = execute(&input);
    println!("part 1 => {}", part_1);
    println!("part 2 => {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let (part_1, part_2) = execute(&input());
        assert_eq!(part_1, "GPALMJSOY");
        assert_eq!(part_2, 16204);
    }
}
