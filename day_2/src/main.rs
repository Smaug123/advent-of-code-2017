use std::cmp::Ordering;
use std::collections::HashSet;

fn input() -> Vec<Vec<u32>> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|i| {
                    i.parse()
                        .unwrap_or_else(|_| panic!("{} wasn't a valid u32", i))
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn min_max<T, I>(i: &mut I) -> Option<(T, T)>
where
    I: Iterator<Item = T>,
    T: Copy + Ord,
{
    if let Some(fst) = i.next() {
        let mut top = fst;
        let mut bot = fst;
        for next in i {
            if next > top {
                top = next;
            }
            if next < bot {
                bot = next;
            }
        }
        Some((bot, top))
    } else {
        None
    }
}

fn even_divisor<I>(iter: &mut I) -> Option<(u32, u32)>
where
    I: Iterator<Item = u32>,
{
    let mut seen: HashSet<u32> = HashSet::new();
    for i in iter {
        for s in &seen {
            let s = *s;
            match u32::cmp(&s, &i) {
                Ordering::Less => {
                    if i % s == 0 {
                        return Some((i, s));
                    };
                }
                Ordering::Greater => {
                    if s % i == 0 {
                        return Some((s, i));
                    };
                }
                Ordering::Equal => {
                    return Some((i, s));
                }
            }
        }
        seen.insert(i);
    }
    None
}

pub fn part_1<I, J>(numbers: &mut I) -> u32
where
    I: Iterator<Item = J>,
    J: Iterator<Item = u32>,
{
    numbers
        .map(|mut row| {
            if let Some((min, max)) = min_max(&mut row) {
                max - min
            } else {
                0
            }
        })
        .sum()
}

pub fn part_2<I, J>(numbers: &mut I) -> u32
where
    I: Iterator<Item = J>,
    J: Iterator<Item = u32>,
{
    numbers
        .map(|mut row| {
            let (bigger, smaller) = even_divisor(&mut row).unwrap();
            bigger / smaller
        })
        .sum()
}

fn main() {
    let input = input();
    println!(
        "part 1 => {}",
        part_1(&mut input.iter().map(|r| r.iter().cloned()))
    );
    println!(
        "part 2 => {}",
        part_2(&mut input.iter().map(|r| r.iter().cloned()))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_known() {
        assert_eq!(
            part_1(
                &mut vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]
                    .iter()
                    .map(|r| r.iter().cloned())
            ),
            18
        );
    }

    #[test]
    fn part2_known() {
        assert_eq!(
            part_2(
                &mut vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]
                    .iter()
                    .map(|r| r.iter().cloned())
            ),
            9
        );
    }

    #[test]
    fn test_day_2() {
        let input = input();
        let answer = part_1(&mut input.iter().map(|r| r.iter().cloned()));
        assert_eq!(answer, 44887);
        let answer = part_2(&mut input.iter().map(|r| r.iter().cloned()));
        assert_eq!(answer, 242);
    }
}
