pub mod day_25 {

    enum Direction {
        Left,
        Right,
    }

    struct Action<Name> {
        write: bool,
        direction: Direction,
        next_state: Name,
    }

    struct Instruction<Name> {
        state_name: Name,
        if_zero: Action<Name>,
        if_one: Action<Name>,
    }

    pub struct TuringMachine {
        transitions: Vec<Instruction<u8>>,
        pause_after: u32,
        starting_state: u8,
    }

    fn extract_string(input: &str, template: &str) -> char {
        let mut chars = input.chars();
        for expected in template.chars() {
            let actual = chars.next().unwrap();
            if expected != actual {
                panic!(
                    "Malformed while matching template, expected: {}, actual: {}",
                    template, input
                );
            }
        }
        chars.next().unwrap()
    }

    fn extract_word<'a>(input: &'a str, template: &[&str]) -> &'a str {
        let mut words = input.split_whitespace();
        for expected in template.iter() {
            let w = words.next().unwrap();
            if expected != &w {
                panic!(
                    "Expected word in template for input '{}': {}; got: {}",
                    input, expected, w
                );
            }
        }
        words.next().unwrap()
    }

    fn peel_action<'a, I, T>(lines: &mut I, state_reduction: fn(char) -> T) -> Action<T>
    where
        I: Iterator<Item = &'a str>,
    {
        let to_write = match extract_string(lines.next().unwrap(), "    - Write the value ") {
            '0' => false,
            '1' => true,
            s => {
                panic!("Expected 0 or 1, got: {}", s);
            }
        };
        let to_move = match extract_word(
            lines.next().unwrap(),
            &["-", "Move", "one", "slot", "to", "the"],
        ) {
            "left." => Direction::Left,
            "right." => Direction::Right,
            s => {
                panic!("Expected a direction, got: {}", s);
            }
        };
        let next_state = extract_string(lines.next().unwrap(), "    - Continue with state ");
        Action {
            direction: to_move,
            next_state: state_reduction(next_state),
            write: to_write,
        }
    }

    impl TuringMachine {
        fn parse(s: &str) -> TuringMachine {
            let mut lines = s.lines();
            let starting_state = extract_string(&lines.next().unwrap(), "Begin in state ");
            let pause_after: u32 = extract_word(
                &lines.next().unwrap(),
                &["Perform", "a", "diagnostic", "checksum", "after"],
            )
            .parse()
            .unwrap();
            let _ = lines.next();

            let mut transitions = vec![];
            loop {
                let state_name = extract_string(lines.next().unwrap(), "In state ");
                let z = extract_string(lines.next().unwrap(), "  If the current value is ")
                    .to_digit(10);
                match z {
                    Some(0) => {}
                    s => {
                        panic!("Expected 0, got: {:?}", s);
                    }
                }
                let if_zero = peel_action(&mut lines, |c| c as u8 - b'A');
                let z = extract_string(lines.next().unwrap(), "  If the current value is ")
                    .to_digit(10);
                match z {
                    Some(1) => {}
                    s => {
                        panic!("Expected 1, got: {:?}", s);
                    }
                }
                let if_one = peel_action(&mut lines, |c| c as u8 - b'A');
                transitions.push(Instruction {
                    state_name: state_name as u8 - b'A',
                    if_zero,
                    if_one,
                });

                match lines.next() {
                    None => {
                        transitions.sort_unstable_by_key(|i| i.state_name);
                        for (pos, t) in transitions.iter().enumerate() {
                            if pos as u8 != t.state_name {
                                panic!("Mis-parse! Didn't densely pack the transition array");
                            }
                        }
                        return TuringMachine {
                            pause_after,
                            starting_state: starting_state as u8 - b'A',
                            transitions,
                        };
                    }
                    Some(_) => {}
                }
            }
        }
    }

    struct ExecutingTuringMachine<'a> {
        state: u8,
        tape_positive: Vec<bool>,
        tape_negative: Vec<bool>,
        head: i32,
        spec: &'a TuringMachine,
    }

    impl ExecutingTuringMachine<'_> {
        fn new(t: &TuringMachine) -> ExecutingTuringMachine {
            ExecutingTuringMachine {
                state: t.starting_state,
                tape_positive: vec![],
                tape_negative: vec![],
                head: 0,
                spec: t,
            }
        }

        fn ones(t: &ExecutingTuringMachine) -> usize {
            t.tape_positive.iter().cloned().filter(|&i| i).count()
                + t.tape_negative.iter().cloned().filter(|&i| i).count()
        }

        fn currently_under_head(t: &ExecutingTuringMachine) -> bool {
            if t.head >= 0 {
                let head = t.head as usize;
                if head >= t.tape_positive.len() {
                    false
                } else {
                    t.tape_positive[head]
                }
            } else {
                let head = (-t.head - 1) as usize;
                if head >= t.tape_negative.len() {
                    false
                } else {
                    t.tape_negative[head]
                }
            }
        }

        fn set_head(t: &mut ExecutingTuringMachine, value: bool) {
            if t.head >= 0 {
                let head = t.head as usize;
                if head >= t.tape_positive.len() {
                    t.tape_positive.push(value);
                } else {
                    t.tape_positive[head] = value;
                }
            } else {
                let head = (-t.head - 1) as usize;
                if head >= t.tape_negative.len() {
                    t.tape_negative.push(value);
                } else {
                    t.tape_negative[head] = value;
                }
            }
        }

        fn execute_action(t: &mut ExecutingTuringMachine, action: &Action<u8>) {
            ExecutingTuringMachine::set_head(t, action.write);
            match action.direction {
                Direction::Left => {
                    t.head -= 1;
                }
                Direction::Right => {
                    t.head += 1;
                }
            }
            t.state = action.next_state;
        }

        fn step(t: &mut ExecutingTuringMachine) {
            let instruction = &t.spec.transitions[t.state as usize];
            if ExecutingTuringMachine::currently_under_head(t) {
                ExecutingTuringMachine::execute_action(t, &instruction.if_one);
            } else {
                ExecutingTuringMachine::execute_action(t, &instruction.if_zero);
            }
        }
    }

    pub fn input() -> TuringMachine {
        let input = include_str!("../input.txt");
        TuringMachine::parse(&input)
    }

    pub fn part_1(tm: &TuringMachine) -> usize {
        let mut tm = ExecutingTuringMachine::new(&tm);
        for _ in 0..tm.spec.pause_after {
            ExecutingTuringMachine::step(&mut tm);
        }

        ExecutingTuringMachine::ones(&tm)
    }
}

#[cfg(test)]
mod tests {
    use super::day_25::*;

    #[test]
    fn test_day_25() {
        let input = input();
        assert_eq!(part_1(&input), 2794);
    }
}
