enum Instruction {
    Set(char, i32),
    SetTo(char, char),
    Sub(char, i32),
    SubBy(char, char),
    Mul(char, i32),
    MulBy(char, char),
    Jnz(char, i32),
    JnzExact(i32, i32),
}

fn parse(s: &str) -> Instruction {
    let mut s = s.split_whitespace();
    match s.next().unwrap() {
        "sub" => {
            let register = {
                let mut chars = s.next().unwrap().chars();
                let result = chars.next().unwrap();
                match chars.next() {
                    Some(i) => panic!("Expected a register, got another char: {}", i),
                    None => result,
                }
            };
            let last = s.next().unwrap();
            match s.next() {
                Some(i) => panic!("Expected no more tokens, got {}", i),
                None => match last.parse::<i32>() {
                    Ok(i) => Instruction::Sub(register, i),
                    Err(_) => {
                        let mut chars = last.chars();
                        let target = chars.next().unwrap();
                        match chars.next() {
                            Some(i) => panic!(
                                "Expected only a one-length register, got another char: {}",
                                i
                            ),
                            None => Instruction::SubBy(register, target),
                        }
                    }
                },
            }
        }
        "set" => {
            let register = {
                let mut chars = s.next().unwrap().chars();
                let result = chars.next().unwrap();
                match chars.next() {
                    Some(i) => panic!("Expected a register, got another char: {}", i),
                    None => result,
                }
            };
            let last = s.next().unwrap();
            match s.next() {
                Some(i) => panic!("Expected no more tokens, got {}", i),
                None => match last.parse::<i32>() {
                    Ok(i) => Instruction::Set(register, i),
                    Err(_) => {
                        let mut chars = last.chars();
                        let target = chars.next().unwrap();
                        match chars.next() {
                            Some(i) => panic!(
                                "Expected only a one-length register, got another char: {}",
                                i
                            ),
                            None => Instruction::SetTo(register, target),
                        }
                    }
                },
            }
        }
        "mul" => {
            let register = {
                let mut chars = s.next().unwrap().chars();
                let result = chars.next().unwrap();
                match chars.next() {
                    Some(i) => panic!("Expected a register, got another char: {}", i),
                    None => result,
                }
            };
            let last = s.next().unwrap();
            match s.next() {
                Some(i) => panic!("Expected no more tokens, got {}", i),
                None => match last.parse::<i32>() {
                    Ok(i) => Instruction::Mul(register, i),
                    Err(_) => {
                        let mut chars = last.chars();
                        let target = chars.next().unwrap();
                        match chars.next() {
                            Some(i) => panic!(
                                "Expected only a one-length register, got another char: {}",
                                i
                            ),
                            None => Instruction::MulBy(register, target),
                        }
                    }
                },
            }
        }
        "jnz" => {
            let register = s.next().unwrap();
            let last = s.next().unwrap().parse::<i32>().unwrap();
            if let Some(i) = s.next() {
                panic!("Expected no more tokens, got {}", i);
            }
            match register.parse::<i32>() {
                Ok(register) => Instruction::JnzExact(register, last),
                Err(_) => {
                    let mut chars = register.chars();
                    let result = chars.next().unwrap();
                    match chars.next() {
                        Some(i) => panic!("Expected a register, got another char: {}", i),
                        None => Instruction::Jnz(result, last),
                    }
                }
            }
        }
        instr => panic!("Unrecognised instruction: {}", instr),
    }
}

fn input() -> Vec<Instruction> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|l| parse(l.trim()))
        .collect::<Vec<Instruction>>()
}

struct Machine<'a> {
    registers: Vec<i32>,
    pc: usize,
    program: &'a [Instruction],
}

fn to_index(i: char) -> usize {
    ((i as u8) - b'a') as usize
}

#[allow(dead_code)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
}

fn return_it(registers: &[i32]) -> Registers {
    Registers {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        d: registers[3],
        e: registers[4],
        f: registers[5],
        g: registers[6],
        h: registers[7],
    }
}

// Performs one step, and returns true for a `mul` and false otherwise.
impl Iterator for Machine<'_> {
    type Item = (bool, Registers);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc >= self.program.len() {
            return None;
        }
        match self.program[self.pc] {
            Instruction::Sub(r, i) => {
                self.registers[to_index(r)] -= i;
                self.pc += 1;
                Some((false, return_it(&self.registers)))
            }
            Instruction::SubBy(r, s) => {
                self.registers[to_index(r)] -= self.registers[to_index(s)];
                self.pc += 1;
                Some((false, return_it(&self.registers)))
            }
            Instruction::Mul(r, i) => {
                self.registers[to_index(r)] *= i;
                self.pc += 1;
                Some((true, return_it(&self.registers)))
            }
            Instruction::MulBy(r, s) => {
                self.registers[to_index(r)] *= self.registers[to_index(s)];
                self.pc += 1;
                Some((true, return_it(&self.registers)))
            }
            Instruction::Set(r, i) => {
                self.registers[to_index(r)] = i;
                self.pc += 1;
                Some((false, return_it(&self.registers)))
            }
            Instruction::SetTo(r, s) => {
                self.registers[to_index(r)] = self.registers[to_index(s)];
                self.pc += 1;
                Some((false, return_it(&self.registers)))
            }
            Instruction::Jnz(r, offset) => {
                if self.registers[to_index(r)] != 0 {
                    let target = self.pc as i32 + offset;
                    if target < 0 || target >= self.program.len() as i32 {
                        None
                    } else {
                        self.pc = target as usize;
                        Some((false, return_it(&self.registers)))
                    }
                } else {
                    self.pc += 1;
                    Some((false, return_it(&self.registers)))
                }
            }
            Instruction::JnzExact(r, offset) => {
                if r != 0 {
                    let target = self.pc as i32 + offset;
                    if target < 0 || target >= self.program.len() as i32 {
                        None
                    } else {
                        self.pc = target as usize;
                        Some((false, return_it(&self.registers)))
                    }
                } else {
                    self.pc += 1;
                    Some((false, return_it(&self.registers)))
                }
            }
        }
    }
}

fn part_1(instructions: &[Instruction]) -> usize {
    let machine = Machine {
        registers: vec![0; 8],
        pc: 0,
        program: instructions,
    };
    machine.filter(|(i, _)| *i).count()
}

#[allow(dead_code)]
fn part_2_dumb(instructions: &[Instruction]) -> i32 {
    let mut registers = vec![0; 8];
    registers[0] = 1;
    let machine = Machine {
        registers,
        pc: 0,
        program: instructions,
    };
    machine.map(|(_, registers)| registers.h).last().unwrap()
}

fn is_composite(i: u32) -> bool {
    if i % 2 == 0 {
        return i != 2;
    }
    if i % 3 == 0 {
        return i != 3;
    }
    let mut factor = 5;
    while factor * factor <= i {
        if i % factor == 0 {
            return true;
        }
        factor += 2;
        if factor * factor <= i && i % factor == 0 {
            return true;
        }
        factor += 4;
    }
    false
}

struct StepUp {
    value: u32,
    increment: u32,
    max: u32,
}

fn new_stepup(b: u32, c: u32, step: u32) -> StepUp {
    StepUp {
        value: b - step,
        increment: step,
        max: c - step,
    }
}

impl Iterator for StepUp {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value > self.max {
            None
        } else {
            self.value += self.increment;
            Some(self.value)
        }
    }
}

fn part_2(instructions: &[Instruction]) -> usize {
    let mut registers = vec![0; 8];
    registers[0] = 1;
    let machine = Machine {
        registers,
        pc: 0,
        program: instructions,
    };
    let (_, registers) = machine.take(10).last().unwrap();

    new_stepup(registers.b as u32, registers.c as u32, 17)
        .filter(|i| is_composite(*i))
        .count()
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&input));
    println!("part 2 => {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_23() {
        let input = input();
        assert_eq!(part_1(&input), 5929);
        assert_eq!(part_2(&input), 907);
    }
}
