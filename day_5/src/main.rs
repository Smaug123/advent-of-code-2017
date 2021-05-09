use std::convert::TryFrom;

fn input() -> Vec<i32> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|l| {
            l.parse()
                .unwrap_or_else(|_| panic!("{} wasn't a valid u32", l))
        })
        .collect::<Vec<i32>>()
}

pub fn part_1(v: &mut Vec<i32>) -> u32 {
    let mut count = 0;
    let mut index: usize = 0;
    while index < v.len() {
        let bounce = v[index];
        v[index] = bounce + 1;
        let test = i32::try_from(index).unwrap() + bounce;
        if test < 0 {
            return count;
        }
        index = usize::try_from(test).unwrap();
        count += 1;
    }
    count
}

pub fn part_2(v: &mut Vec<i32>) -> u32 {
    let mut count = 0;
    let mut index: usize = 0;
    while index < v.len() {
        let bounce = v[index];
        if bounce >= 3 {
            v[index] = bounce - 1;
        } else {
            v[index] = bounce + 1;
        }
        let test = i32::try_from(index).unwrap() + bounce;
        if test < 0 {
            return count;
        }
        index = usize::try_from(test).unwrap();
        count += 1;
    }
    count
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&mut input.clone()));
    let mut input = input;
    println!("part 2 => {}", part_2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&mut vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&mut vec![0, 3, 0, 1, -3]), 10);
    }

    #[test]
    fn test_day_5() {
        let input = input();
        assert_eq!(part_1(&mut input.clone()), 391540);
        let mut input = input;
        assert_eq!(part_2(&mut input), 30513679);
    }
}
