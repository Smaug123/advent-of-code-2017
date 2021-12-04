pub mod day_20 {

    use std::cmp::Ordering;

    #[derive(PartialEq, Eq)]
    pub struct Vector {
        x: i16,
        y: i16,
        z: i16,
    }

    const fn abs(v: &Vector) -> i16 {
        v.x * v.x + v.y * v.y + v.z * v.z
    }

    impl PartialOrd for Vector {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(i16::cmp(&abs(self), &abs(other)))
        }
    }

    impl Ord for Vector {
        fn cmp(&self, other: &Self) -> Ordering {
            Vector::partial_cmp(&self, &other).unwrap()
        }
    }

    #[derive(PartialEq, Eq)]
    pub struct Particle {
        position: Vector,
        velocity: Vector,
        acceleration: Vector,
    }

    impl PartialOrd for Particle {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match Vector::cmp(&self.acceleration, &other.acceleration) {
                Ordering::Less => {
                    return Some(Ordering::Less);
                }
                Ordering::Greater => {
                    return Some(Ordering::Greater);
                }
                Ordering::Equal => {}
            }
            match Vector::cmp(&self.velocity, &other.velocity) {
                Ordering::Less => {
                    return Some(Ordering::Less);
                }
                Ordering::Greater => {
                    return Some(Ordering::Greater);
                }
                Ordering::Equal => {}
            }
            Vector::partial_cmp(&self.position, &other.position)
        }
    }

    impl Ord for Particle {
        fn cmp(&self, other: &Self) -> Ordering {
            Particle::partial_cmp(&self, &other).unwrap()
        }
    }

    fn consume<I>(s: &mut I, expected: &str)
    where
        I: Iterator<Item = char>,
    {
        for c in expected.chars() {
            match s.next() {
                Some(t) => {
                    if t != c {
                        panic!("Expected character {}, got {}, in {}", c, t, expected);
                    }
                }
                None => panic!("Expected a character"),
            }
        }
    }

    fn chomp_int<I>(s: &mut I) -> i16
    where
        I: Iterator<Item = char>,
    {
        let first_char = s.next().unwrap();
        let (is_negative, first_char) = if first_char == '-' {
            (true, s.next().unwrap())
        } else {
            (false, first_char)
        };

        let mut answer = char::to_digit(first_char, 10).unwrap() as i16;

        loop {
            match s.next() {
                Some(d) => match char::to_digit(d, 10) {
                    Some(d) => {
                        answer = answer * 10 + (d as i16);
                    }
                    None => {
                        return answer * if is_negative { -1 } else { 1 };
                    }
                },
                None => {
                    panic!("Expected a character!");
                }
            }
        }
    }

    fn chomp<I>(s: &mut I, expected_first: char) -> Vector
    where
        I: Iterator<Item = char>,
    {
        match s.next() {
            Some(t) => {
                if t != expected_first {
                    panic!("Expected first character {}, got {}", expected_first, t);
                }
            }
            None => panic!("Expected a first!"),
        }
        consume(s, "=<");
        let x = chomp_int(s);
        let y = chomp_int(s);
        let z = chomp_int(s);
        Vector { x, y, z }
    }

    fn parse(s: &str) -> Particle {
        let mut s = s.chars();
        let position = chomp(&mut s, 'p');
        consume(&mut s, ", ");
        let velocity = chomp(&mut s, 'v');
        consume(&mut s, ", ");
        let acceleration = chomp(&mut s, 'a');
        match s.next() {
            None => {
                return Particle {
                    position,
                    velocity,
                    acceleration,
                }
            }
            Some(c) => {
                panic!("Expected EOL, got {}", c);
            }
        }
    }

    pub fn input() -> Vec<Particle> {
        let input = include_str!("../input.txt");
        input
            .lines()
            .map(|l| parse(l.trim()))
            .collect::<Vec<Particle>>()
    }

    pub fn part_1(instructions: &[Particle]) -> usize {
        // We need the particle with the least absolute acceleration.
        // If there is a tie, we need the particle with the least absolute velocity.
        // If they are still tied, we need the particle with the least absolute position.
        let (index, _) = instructions.iter().enumerate().min_by_key(|x| x.1).unwrap();
        index
    }

    pub fn part_2(_instructions: &[Particle]) -> usize {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::day_20::*;

    #[test]
    fn test_day_20() {
        let input = input();
        assert_eq!(part_1(&input), 243);
        assert_eq!(part_2(&input), 907);
    }
}
