use std::collections::HashMap;

enum Instruction {
    Spin(u8),
    Exchange(u8, u8),
    Swap(char, char),
}

fn parse(s: &str) -> Instruction {
    let mut chars = s.chars();
    match chars.next().unwrap() {
        's' => Instruction::Spin(chars.as_str().parse().unwrap()),
        'p' => {
            let one = chars.next().unwrap();
            let slash = chars.next().unwrap();
            if slash != '/' {
                panic!("Expected slash, got {}", slash);
            }
            let two = chars.next().unwrap();
            Instruction::Swap(one, two)
        }
        'x' => {
            let mut components = chars.as_str().split('/');
            let one = components.next().unwrap().parse().unwrap();
            let two = components.next().unwrap().parse().unwrap();
            match components.next() {
                Some(s) => panic!("Expected no more components, got {}", s),
                None => Instruction::Exchange(one, two),
            }
        }
        s => panic!("Unexpected char: {}", s),
    }
}

fn input() -> Vec<Instruction> {
    let input = include_str!("../input.txt");
    input
        .split_terminator(',')
        .map(|s| parse(s.trim()))
        .collect::<Vec<_>>()
}

const ASCII_A: u8 = 97;

fn part_1_step(
    program: &mut Vec<u8>,
    character_to_index: &mut Vec<usize>,
    instructions: &[Instruction],
) {
    let size = program.len();
    for instr in instructions {
        match instr {
            Instruction::Spin(offset) => {
                // Everything in location x must now go to x-i
                for (i, loc) in character_to_index.iter_mut().enumerate() {
                    let new_index = (*loc + (*offset as usize)) % size;
                    *loc = new_index;
                    program[new_index] = i as u8 + ASCII_A;
                }
            }
            Instruction::Swap(i, j) => {
                let index_i = (*i as u8 - ASCII_A) as usize;
                let index_j = (*j as u8 - ASCII_A) as usize;

                character_to_index.swap(index_i, index_j);

                program[character_to_index[index_i]] = *i as u8;
                program[character_to_index[index_j]] = *j as u8;
            }
            Instruction::Exchange(i, j) => {
                let char_i = program[*i as usize];
                let char_j = program[*j as usize];
                program[*i as usize] = char_j;
                program[*j as usize] = char_i;

                character_to_index[(char_i as u8 - ASCII_A) as usize] = *j as usize;
                character_to_index[(char_j as u8 - ASCII_A) as usize] = *i as usize;
            }
        }
    }
}

fn part_1(size: usize, instructions: &[Instruction]) -> String {
    let mut program = vec![0u8; size];
    let mut character_to_index = vec![0; size];
    for (i, loc) in program.iter_mut().enumerate() {
        character_to_index[i] = i;
        *loc = i as u8 + ASCII_A;
    }

    part_1_step(&mut program, &mut character_to_index, instructions);
    String::from_utf8(program).unwrap()
}

fn part_2(size: usize, instructions: &[Instruction]) -> String {
    let mut program = vec![0u8; size];
    let mut character_to_index = vec![0; size];
    for (i, loc) in program.iter_mut().enumerate() {
        character_to_index[i] = i;
        *loc = i as u8 + ASCII_A;
    }
    const MAX: usize = 1000000000;

    let mut seen: HashMap<String, usize> = HashMap::new();
    seen.insert(String::from_utf8(program.clone()).unwrap(), 0);

    let period = (1..=MAX)
        .filter_map(|i| {
            part_1_step(&mut program, &mut character_to_index, instructions);
            let cloned = String::from_utf8(program.clone()).unwrap();
            seen.insert(cloned, i).map(|old| (i - old, i))
        })
        .next();

    match period {
        None => String::from_utf8(program).unwrap(),
        Some((period, start)) => {
            let iterations = (MAX - start) % period;
            for _ in 0..iterations {
                part_1_step(&mut program, &mut character_to_index, instructions);
            }
            String::from_utf8(program).unwrap()
        }
    }
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(16, &input));
    println!("part 2 => {}", part_2(16, &input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_known() {
        let instructions: Vec<Instruction> = vec!["s1", "x3/4", "pe/b"]
            .iter()
            .map(|i| parse(i))
            .collect();
        assert_eq!(part_1(5, &instructions), "baedc");
    }

    #[test]
    fn test_day_1() {
        let input = input();
        assert_eq!(part_1(16, &input), "ebjpfdgmihonackl");
        assert_eq!(part_2(16, &input), "abocefghijklmndp");
    }
}
