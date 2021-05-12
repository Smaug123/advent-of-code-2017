enum Instruction {
    Set(char, i64),
    SetTo(char, char),
    Add(char, i64),
    AddTo(char, char),
    Mul(char, i64),
    MulBy(char, char),
    Mod(char, i64),
    ModBy(char, char),
    JgzRegVal(char, i64),
    JgzValVal(i64, i64),
    JgzRegReg(char, char),
    JgzValReg(i64, char),
    Snd(char),
    SndExact(i64),
    Rcv(char),
}

fn parse(s: &str) -> Instruction {
    let mut s = s.split_whitespace();
    match s.next().unwrap() {
        "add" => {
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
                None => match last.parse::<i64>() {
                    Ok(i) => Instruction::Add(register, i),
                    Err(_) => {
                        let mut chars = last.chars();
                        let target = chars.next().unwrap();
                        match chars.next() {
                            Some(i) => panic!(
                                "Expected only a one-length register, got another char: {}",
                                i
                            ),
                            None => Instruction::AddTo(register, target),
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
                None => match last.parse::<i64>() {
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
                None => match last.parse::<i64>() {
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
        "mod" => {
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
                None => match last.parse::<i64>() {
                    Ok(i) => Instruction::Mod(register, i),
                    Err(_) => {
                        let mut chars = last.chars();
                        let target = chars.next().unwrap();
                        match chars.next() {
                            Some(i) => panic!(
                                "Expected only a one-length register, got another char: {}",
                                i
                            ),
                            None => Instruction::ModBy(register, target),
                        }
                    }
                },
            }
        }
        "jgz" => {
            let register = s.next().unwrap();
            let last = s.next().unwrap();
            if let Some(i) = s.next() {
                panic!("Expected no more tokens, got {}", i);
            }
            match register.parse::<i64>() {
                Ok(register) => match last.parse() {
                    Ok(last) => Instruction::JgzValVal(register, last),
                    Err(_) => {
                        let mut chars = last.chars();
                        let last = chars.next().unwrap();
                        if let Some(i) = chars.next() {
                            panic!(
                                "Expected a register name for last token, got another char: {}",
                                i
                            );
                        }
                        Instruction::JgzValReg(register, last)
                    }
                },
                Err(_) => {
                    let mut chars = register.chars();
                    let result = chars.next().unwrap();
                    if let Some(i) = chars.next() {
                        panic!("Expected a register, got another char: {}", i);
                    }
                    match last.parse::<i64>() {
                        Ok(last) => Instruction::JgzRegVal(result, last),
                        Err(_) => {
                            let mut chars = last.chars();
                            let last = chars.next().unwrap();
                            if let Some(i) = chars.next() {
                                panic!("Expected a register name for last, got extra: {}", i);
                            }
                            Instruction::JgzRegReg(result, last)
                        }
                    }
                }
            }
        }
        "snd" => {
            let register = s.next().unwrap();
            if let Some(i) = s.next() {
                panic!("Expected no more tokens, got {}", i);
            }
            match register.parse::<i64>() {
                Ok(i) => Instruction::SndExact(i),
                Err(_) => {
                    let mut chars = register.chars();
                    let register = chars.next().unwrap();
                    if let Some(i) = chars.next() {
                        panic!("Expected a single register, got another character: {}", i);
                    }
                    Instruction::Snd(register)
                }
            }
        }
        "rcv" => {
            let register = s.next().unwrap();
            if let Some(i) = s.next() {
                panic!("Expected no more tokens, got {}", i);
            }
            let mut chars = register.chars();
            let register = chars.next().unwrap();
            if let Some(i) = chars.next() {
                panic!("Expected a single register, got another character: {}", i);
            }
            Instruction::Rcv(register)
        }
        instr => panic!("Unrecognised instruction: {}", instr),
    }
}

struct Machine<'a> {
    program: &'a [Instruction],
    pc: usize,
    registers: &'a mut Vec<i64>,
}

enum MachineState {
    Send(i64),
    Blocked,
    Received(i64),
}

struct Queue<T> {
    elts: Vec<T>,
    ptr: usize,
}

fn new_queue<T>() -> Queue<T> {
    Queue {
        elts: vec![],
        ptr: 1,
    }
}

// Basically an iterator, really.
fn advance<'a>(m: &mut Machine<'a>, receive_queue: &mut Queue<i64>) -> Option<MachineState> {
    loop {
        match m.program[m.pc] {
            Instruction::Add(register, value) => {
                m.registers[(register as u8 - b'a') as usize] += value;
                m.pc += 1;
            }
            Instruction::AddTo(register, other) => {
                m.registers[(register as u8 - b'a') as usize] +=
                    m.registers[(other as u8 - b'a') as usize];
                m.pc += 1;
            }
            Instruction::Mul(register, value) => {
                m.registers[(register as u8 - b'a') as usize] *= value;
                m.pc += 1;
            }
            Instruction::MulBy(register, other) => {
                m.registers[(register as u8 - b'a') as usize] *=
                    m.registers[(other as u8 - b'a') as usize];
                m.pc += 1;
            }
            Instruction::Mod(register, value) => {
                m.registers[(register as u8 - b'a') as usize] %= value;
                m.pc += 1;
            }
            Instruction::ModBy(register, other) => {
                m.registers[(register as u8 - b'a') as usize] %=
                    m.registers[(other as u8 - b'a') as usize];
                m.pc += 1;
            }
            Instruction::Set(register, value) => {
                m.registers[(register as u8 - b'a') as usize] = value;
                m.pc += 1;
            }
            Instruction::SetTo(register, other) => {
                m.registers[(register as u8 - b'a') as usize] =
                    m.registers[(other as u8 - b'a') as usize];
                m.pc += 1;
            }
            Instruction::JgzRegReg(register, offset) => {
                if m.registers[(register as u8 - b'a') as usize] > 0 {
                    let dest = m.pc as i64 + m.registers[(offset as u8 - b'a') as usize];
                    if dest >= m.program.len() as i64 || dest < 0 {
                        return None;
                    } else {
                        m.pc = dest as usize;
                    }
                } else {
                    m.pc += 1;
                }
            }
            Instruction::JgzValReg(cmp, offset) => {
                if cmp > 0 {
                    let dest = m.pc as i64 + m.registers[(offset as u8 - b'a') as usize];
                    if dest >= m.program.len() as i64 || dest < 0 {
                        return None;
                    } else {
                        m.pc = dest as usize;
                    }
                } else {
                    m.pc += 1;
                }
            }
            Instruction::JgzRegVal(register, value) => {
                if m.registers[(register as u8 - b'a') as usize] > 0 {
                    let dest = m.pc as i64 + value;
                    if dest >= m.program.len() as i64 || dest < 0 {
                        return None;
                    } else {
                        m.pc = dest as usize;
                    }
                } else {
                    m.pc += 1;
                }
            }
            Instruction::JgzValVal(cmp, value) => {
                if cmp > 0 {
                    let dest = m.pc as i64 + value;
                    if dest < 0 {
                        return None;
                    } else {
                        m.pc = dest as usize;
                    }
                } else {
                    m.pc += 1;
                }
            }
            Instruction::Snd(value) => {
                m.pc += 1;
                return Some(MachineState::Send(
                    m.registers[(value as u8 - b'a') as usize],
                ));
            }
            Instruction::SndExact(value) => {
                m.pc += 1;
                return Some(MachineState::Send(value));
            }
            Instruction::Rcv(value) => {
                if receive_queue.ptr > receive_queue.elts.len() {
                    return Some(MachineState::Blocked);
                } else {
                    let received = receive_queue.elts[receive_queue.ptr - 1];
                    m.registers[(value as u8 - b'a') as usize] = received;
                    receive_queue.ptr += 1;
                    m.pc += 1;
                    return Some(MachineState::Received(received));
                }
            }
        }
        if m.pc >= m.program.len() {
            return None;
        }
    }
}

fn input() -> Vec<Instruction> {
    let input = include_str!("../input.txt");
    input.lines().map(|l| parse(l)).collect()
}

fn part_1(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine {
        pc: 0,
        program: instructions,
        registers: &mut vec![0; 26],
    };
    let mut queue_1 = new_queue();
    while let Some(state) = advance(&mut machine, &mut queue_1) {
        match state {
            MachineState::Blocked => {
                panic!("Shouldn't have deadlocked.");
            }
            MachineState::Send(i) => {
                queue_1.elts.push(i);
            }
            MachineState::Received(_) => {
                return *queue_1.elts.last().unwrap();
            }
        }
    }
    panic!("Expected to send a value");
}

fn part_2(instructions: &[Instruction]) -> usize {
    let mut machine0 = Machine {
        pc: 0,
        program: instructions,
        registers: &mut vec![0; 26],
    };
    let mut machine1 = Machine {
        pc: 0,
        program: instructions,
        registers: &mut vec![0; 26],
    };
    machine1.registers[(b'p' - b'a') as usize] = 1;

    let mut machine_0_done = false;
    let mut queue_0 = new_queue();
    let mut queue_1 = new_queue();

    // We need the number of times machine 1 sent a value;
    // that is, the final length of queue 0.
    'machine1: while let Some(state) = advance(&mut machine1, &mut queue_1) {
        match state {
            MachineState::Blocked => {
                let mut may_deadlock = true;
                if machine_0_done {
                    // Deadlock!
                    return queue_0.elts.len();
                } else {
                    while let Some(state) = advance(&mut machine0, &mut queue_0) {
                        match state {
                            MachineState::Blocked => {
                                if may_deadlock {
                                    return queue_0.elts.len();
                                } else {
                                    // Suspend machine 0.
                                    continue 'machine1;
                                }
                            }
                            MachineState::Send(i) => {
                                // We no longer risk deadlock, because machine 1 has input again.
                                may_deadlock = false;
                                queue_1.elts.push(i);
                            }
                            MachineState::Received(_) => {}
                        }
                    }
                    machine_0_done = true;
                }
            }
            MachineState::Send(i) => {
                queue_0.elts.push(i);
            }
            MachineState::Received(_) => {}
        }
    }
    queue_0.elts.len()
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&input));
    println!("part 2 => {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<&'static str> {
        vec![
            "set a 1", "add a 2", "mul a a", "mod a 5", "snd a", "set a 0", "rcv a", "jgz a -1",
            "set a 1", "jgz a -2",
        ]
    }

    #[test]
    fn part1_known() {
        let input: Vec<Instruction> = test_input().iter().map(|i| parse(i)).collect();
        assert_eq!(part_1(&input), 4);
    }

    fn test_input_2() -> Vec<&'static str> {
        vec![
            "snd 1", "snd 2", "snd p", "rcv a", "rcv b", "rcv c", "rcv d",
        ]
    }

    #[test]
    fn part2_known() {
        let input: Vec<Instruction> = test_input_2().iter().map(|i| parse(i)).collect();
        assert_eq!(part_2(&input), 3)
    }

    #[test]
    fn test_day_18() {
        let input = input();
        assert_eq!(part_1(&input), 3423);
        assert_eq!(part_2(&input), 7493);
    }
}
