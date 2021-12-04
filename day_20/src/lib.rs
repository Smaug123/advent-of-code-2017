pub mod day_20 {

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::ops::{Add, Mul};

    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct Vector<T> {
        x: T,
        y: T,
        z: T,
    }

    fn abs<T>(v: &Vector<T>) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        v.x * v.x + v.y * v.y + v.z * v.z
    }

    impl<T> PartialOrd for Vector<T>
    where
        T: Ord,
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(T::cmp(&abs(self), &abs(other)))
        }
    }

    impl<T> Ord for Vector<T>
    where
        T: Ord,
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            Vector::partial_cmp(self, other).unwrap()
        }
    }

    impl<'a, T> Add<&'a Vector<T>> for &'a Vector<T>
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        type Output = Vector<T>;
        fn add(self, other: Self) -> Vector<T> {
            Vector {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct Particle<T> {
        position: Vector<T>,
        velocity: Vector<T>,
        acceleration: Vector<T>,
    }

    impl<T> PartialOrd for Particle<T>
    where
        T: Ord,
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
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

    impl<T> Ord for Particle<T>
    where
        T: Ord,
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            Particle::partial_cmp(self, other).unwrap()
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

    fn chomp_int<I>(s: &mut I) -> i32
    where
        I: Iterator<Item = char>,
    {
        let first_char = s.next().unwrap();
        let (is_negative, first_char) = if first_char == '-' {
            (true, s.next().unwrap())
        } else {
            (false, first_char)
        };

        let mut answer = char::to_digit(first_char, 10).unwrap() as i32;

        loop {
            match s.next() {
                Some(d) => match char::to_digit(d, 10) {
                    Some(d) => {
                        answer = answer * 10 + (d as i32);
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

    fn chomp<I>(s: &mut I, expected_first: char) -> Vector<i32>
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

    fn parse(s: &str) -> Particle<i32> {
        let mut s = s.chars();
        let position = chomp(&mut s, 'p');
        consume(&mut s, ", ");
        let velocity = chomp(&mut s, 'v');
        consume(&mut s, ", ");
        let acceleration = chomp(&mut s, 'a');
        match s.next() {
            None => Particle {
                position,
                velocity,
                acceleration,
            },
            Some(c) => {
                panic!("Expected EOL, got {}", c);
            }
        }
    }

    pub fn input() -> Vec<Particle<i32>> {
        let input = include_str!("../input.txt");
        input.lines().map(|l| parse(l.trim())).collect::<Vec<_>>()
    }

    pub fn part_1<T>(particles: &[Particle<T>]) -> usize
    where
        T: Ord,
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        // We need the particle with the least absolute acceleration.
        // If there is a tie, we need the particle with the least absolute velocity.
        // If they are still tied, we need the particle with the least absolute position.
        let (index, _) = particles.iter().enumerate().min_by_key(|x| x.1).unwrap();
        index
    }

    fn tick_one<T>(particle: &mut Particle<T>)
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        particle.velocity = &particle.velocity + &particle.acceleration;
        particle.position = &particle.position + &particle.velocity;
    }

    fn tick<T>(particles: &mut [(bool, &mut Particle<T>)])
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
        T: Clone,
        T: Eq,
        T: Hash,
    {
        for (is_gone, particle) in particles.iter_mut() {
            if !*is_gone {
                tick_one(particle);
            }
        }

        let mut seen = HashMap::new();

        for i in 0..particles.len() {
            if !particles[i].0 {
                match seen.insert(particles[i].1.clone(), i) {
                    None => {}
                    Some(old_index) => {
                        particles[old_index].0 = true;
                        particles[i].0 = true;
                    }
                }
            }
        }
    }

    pub fn part_2<T>(particles: &[Particle<T>]) -> usize
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Clone,
        T: Copy,
        T: Eq,
        T: Hash,
    {
        let mut particles = particles.to_vec();
        let mut positions = particles.iter_mut().map(|p| (false, p)).collect::<Vec<_>>();

        for _ in 0..1000 {
            tick(&mut positions);
        }

        positions.iter().filter(|(is_gone, _)| !is_gone).count()
    }
}

#[cfg(test)]
mod tests {
    use super::day_20::*;

    #[test]
    fn test_day_20() {
        let input = input();
        assert_eq!(part_1(&input), 243);
        assert_eq!(part_2(&input), 0);
    }
}
