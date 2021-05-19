pub mod day_14 {

    use day_10::day_10::knot_hash;

    pub fn input() -> &'static str {
        include_str!("../input.txt").trim()
    }

    fn count_ones(i: u8) -> u8 {
        let mut i = i;
        let mut ans = 0;
        while i > 0 {
            if i % 2 == 1 {
                ans += 1;
            }
            i /= 2;
        }
        ans
    }

    pub fn part_1_longhand(key: &str) -> u32 {
        let mut buffer: Vec<u8> = key.chars().map(|i| i as u8).collect();
        buffer.extend(&[b'-', b'0']);
        let len = buffer.len();

        let mut count = 0u32;

        for i in 0u8..=9 {
            buffer[len - 1] = i + b'0';
            let hash = knot_hash(&buffer);
            count += hash
                .iter()
                .cloned()
                .map(|i| count_ones(i) as u32)
                .sum::<u32>();
        }

        buffer.push(0);
        for i in 1u8..=9 {
            buffer[len - 1] = i + b'0';
            for j in 0u8..=9 {
                buffer[len] = j + b'0';
                count += knot_hash(&buffer)
                    .iter()
                    .cloned()
                    .map(|i| count_ones(i) as u32)
                    .sum::<u32>();
            }
        }

        buffer.push(0);
        buffer[len - 1] = b'1';
        for i in 0u8..=2 {
            buffer[len] = i + b'0';
            for j in 0u8..=(if i == 2 { 7 } else { 9 }) {
                buffer[len + 1] = j + b'0';
                count += knot_hash(&buffer)
                    .iter()
                    .cloned()
                    .map(|i| count_ones(i) as u32)
                    .sum::<u32>();
            }
        }

        count
    }

    fn value_at<T>(i: &[T], width: usize, row: usize, col: usize) -> &T {
        &i[row * width + col]
    }

    fn set_value_at<T>(i: &mut [T], width: usize, row: usize, col: usize, value: T) {
        i[row * width + col] = value;
    }

    fn flood_fill(i: &mut [u32], width: usize, value: u32, row: usize, col: usize) {
        let current = *value_at(i, width, row, col);
        if current != 1 {
            return;
        }
        set_value_at(i, width, row, col, value);
        if row < (i.len() / width) - 1 {
            flood_fill(i, width, value, row + 1, col);
        }
        if row > 0 {
            flood_fill(i, width, value, row - 1, col);
        }
        if col > 0 {
            flood_fill(i, width, value, row, col - 1);
        }
        if col < width - 1 {
            flood_fill(i, width, value, row, col + 1);
        }
    }

    fn count_contiguous(mut i: Vec<u32>, width: usize) -> u32 {
        // 0 means empty, 1 means "full and not assigned a group".
        let height = i.len() / width;
        let mut next_group_number = 2;
        for row in 0..height {
            for col in 0..width {
                if *value_at(&i, width, row, col) == 1 {
                    flood_fill(&mut i, width, next_group_number, row, col);
                    next_group_number += 1;
                }
            }
        }
        next_group_number - 2
    }

    pub(crate) fn render_vec(key: &str) -> Vec<u32> {
        let mut output: Vec<u32> = vec![0; 128 * 128];
        let mut buffer: Vec<u8> = key.chars().map(|i| i as u8).collect();
        buffer.extend(&[b'-', b'0']);
        let len = buffer.len();

        let mut row = 0;

        for i in 0u8..=9 {
            buffer[len - 1] = i + b'0';
            let hash = knot_hash(&buffer);
            let mut col = 0;
            for byte in hash {
                let mut byte = byte;
                for i in (0..8).rev() {
                    set_value_at(&mut output, 128, row, col + i, (byte % 2) as u32);
                    byte /= 2;
                }
                col += 8;
            }
            row += 1;
        }

        buffer.push(0);
        for i in 1u8..=9 {
            buffer[len - 1] = i + b'0';
            for j in 0u8..=9 {
                buffer[len] = j + b'0';
                let hash = knot_hash(&buffer);
                let mut col = 0;
                for byte in hash {
                    let mut byte = byte;
                    for i in (0..8).rev() {
                        set_value_at(&mut output, 128, row, col + i, (byte % 2) as u32);
                        byte /= 2;
                    }
                    col += 8;
                }
                row += 1;
            }
        }

        buffer.push(0);
        buffer[len - 1] = b'1';
        for i in 0u8..=2 {
            buffer[len] = i + b'0';
            for j in 0u8..=(if i == 2 { 7 } else { 9 }) {
                buffer[len + 1] = j + b'0';
                let hash = knot_hash(&buffer);
                let mut col = 0;
                for byte in hash {
                    let mut byte = byte;
                    for i in (0..8).rev() {
                        set_value_at(&mut output, 128, row, col + i, (byte % 2) as u32);
                        byte /= 2;
                    }
                    col += 8;
                }
                row += 1;
            }
        }

        output
    }

    pub fn part_1(key: &str) -> usize {
        render_vec(key).iter().cloned().filter(|i| *i == 1).count()
    }

    pub fn part_2(key: &str) -> u32 {
        count_contiguous(render_vec(key), 128)
    }
}

#[cfg(test)]
mod tests {
    use super::day_14::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(&"flqrgnkx"), 8108);
        assert_eq!(part_1_longhand(&"flqrgnkx"), 8108);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(&"flqrgnkx"), 1242);
    }

    #[test]
    fn test_day_14() {
        let input = input();
        assert_eq!(part_1(input), 8190);
        assert_eq!(part_1_longhand(input), 8190);
        assert_eq!(part_2(&input), 1134);
    }
}
