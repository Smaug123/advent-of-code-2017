use std::collections::HashMap;

enum Operation {
    Incr,
    Decr,
}

enum Comparison {
    Greater,
    Equal,
    Less,
    LessEqual,
    GreaterEqual,
    NotEqual,
}

struct Condition<'a> {
    register: &'a str,
    comparison: Comparison,
    number: i32,
}

struct Instruction<'a> {
    register: &'a str,
    op: Operation,
    amount: i32,
    condition: Condition<'a>,
}

fn parse(s: &str) -> Instruction {
    let mut iter = s.split_whitespace();
    let register = iter.next().unwrap();
    let op = match iter.next().unwrap() {
        "inc" => Operation::Incr,
        "dec" => Operation::Decr,
        s => panic!("Bad! {}", s),
    };
    let amount: i32 = iter.next().unwrap().parse().unwrap();
    let if_clause = iter.next().unwrap();
    if if_clause != "if" {
        panic!("Expected 'if', got: {}", if_clause);
    }
    let cmp_variable = iter.next().unwrap();
    let cmp_operator = match iter.next().unwrap() {
        "<=" => Comparison::LessEqual,
        ">=" => Comparison::GreaterEqual,
        "==" => Comparison::Equal,
        "<" => Comparison::Less,
        ">" => Comparison::Greater,
        "!=" => Comparison::NotEqual,
        s => panic!("Expected comparison, got: {}", s),
    };
    let cmp_num: i32 = iter.next().unwrap().parse().unwrap();
    let condition = Condition {
        register: cmp_variable,
        comparison: cmp_operator,
        number: cmp_num,
    };
    match iter.next() {
        None => Instruction {
            register,
            op,
            amount,
            condition,
        },
        Some(s) => panic!("Exp ected end of line, got {}", s),
    }
}

fn input() -> Vec<Instruction<'static>> {
    let input = include_str!("../input.txt");
    input.lines().map(|l| parse(l)).collect::<Vec<_>>()
}

fn process<'a>(instructions: &[Instruction<'a>]) -> HashMap<&'a str, (i32, i32)> {
    let mut registers = HashMap::new();
    for instruction in instructions {
        let original = registers
            .get(instruction.condition.register)
            .map_or(0, |&(_, v)| v);
        let proceed = match instruction.condition.comparison {
            Comparison::Greater => original > instruction.condition.number,
            Comparison::Less => original < instruction.condition.number,
            Comparison::GreaterEqual => original >= instruction.condition.number,
            Comparison::LessEqual => original <= instruction.condition.number,
            Comparison::Equal => original == instruction.condition.number,
            Comparison::NotEqual => original != instruction.condition.number,
        };
        if proceed {
            let (original_max, original_val) =
                registers.get(instruction.register).map_or((0, 0), |i| *i);
            let new_val = match instruction.op {
                Operation::Incr => original_val + instruction.amount,
                Operation::Decr => original_val - instruction.amount,
            };
            let new_max = std::cmp::max(original_max, new_val);
            registers.insert(instruction.register, (new_max, new_val));
        }
    }
    registers
}

fn part_1(instructions: &[Instruction]) -> i32 {
    let map = process(instructions);
    map.values().map(|&(_, i)| i).max().unwrap()
}

fn part_2(instructions: &[Instruction]) -> i32 {
    let map = process(instructions);
    map.values().map(|&(i, _)| i).max().unwrap()
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&input));
    println!("part 2 => {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<Instruction<'static>> {
        vec![
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ]
        .iter()
        .map(|s| parse(s))
        .collect()
    }

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&data()), 1);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&data()), 10);
    }

    #[test]
    fn test_day_1() {
        let input = input();
        assert_eq!(part_1(&input), 5752);
        assert_eq!(part_2(&input), 6366);
    }
}
