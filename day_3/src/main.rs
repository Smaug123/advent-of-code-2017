fn input() -> u32 {
    let input = include_str!("../input.txt");
    input
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} wasn't a valid u32", input))
}

fn layer(input: u32) -> u32 {
    f32::ceil((f32::sqrt(input as f32) + 1.0) / 2.0) as u32
}

pub fn part_1(input: u32) -> u32 {
    if input == 1 {
        return 0;
    }
    // Which layer are we in?
    let layer = layer(input);
    // Where did this layer start?
    let start = (2 * layer - 3) * (2 * layer - 3) + 1;
    let right_middle = start + layer - 2;
    if input <= right_middle {
        return right_middle - input + layer - 1;
    }
    let right_top = right_middle + layer - 1;
    if input <= right_top {
        return input - right_middle + layer - 1;
    }
    let middle_top = right_top + layer - 1;
    if input <= middle_top {
        return middle_top - input + layer - 1;
    }
    let left_top = middle_top + layer - 1;
    if input <= left_top {
        return input - middle_top + layer - 1;
    }
    let left_middle = left_top + layer - 1;
    if input <= left_middle {
        return left_middle - input + layer - 1;
    }
    let left_bottom = left_middle + layer - 1;
    if input <= left_bottom {
        return input - left_middle + layer - 1;
    }
    let middle_bottom = left_bottom + layer - 1;
    if input <= middle_bottom {
        return middle_bottom - input + layer - 1;
    }
    input - middle_bottom + layer - 1
}

fn layer_size(layer: usize) -> usize {
    if layer == 1 {
        1
    } else {
        let s = 2 * layer - 1;
        let t = 2 * (layer - 1) - 1;
        (s * s) - (t * t)
    }
}

#[cfg(test)]
mod layer_test {
    use super::*;

    #[test]
    fn layer_test_1() {
        assert_eq!(layer_size(1), 1);
        assert_eq!(layer_size(2), 8);
        assert_eq!(layer_size(3), 16);
    }
}

#[macro_export]
macro_rules! set_or_return {
    ( $x:expr, $input:ident, $val: expr) => {{
        $x = $val;
        if $x > $input {
            return $x;
        }
    }};
}

pub fn part_2(input: u32) -> u32 {
    let mut prev_layer: Vec<u32> = vec![1, 2, 4, 5, 10, 11, 23, 25];
    if input < 25 {
        return prev_layer.iter().cloned().find(|&i| i > input).unwrap();
    }

    for layer in 3.. {
        let mut curr_layer = vec![0; layer_size(layer)];

        // Starting one place above the bottom-right, fill in anticlockwise.
        // The right-hand edge, where for the purposes of this loop only,
        // our `i` is viewing itself as ranging over the actual indices.
        // So i = 1 means we're one step above the bottom of the layer (i.e. it's
        // the smallest element in the layer).
        // As this loop views things, i = 0 would be the largest element in the layer.
        // Note, though, that `curr_layer` is still indexed from 0 being the smallest
        // element.
        set_or_return!(
            curr_layer[0],
            input,
            prev_layer[0] + prev_layer[prev_layer.len() - 1]
        );
        set_or_return!(
            curr_layer[1],
            input,
            prev_layer[0] + prev_layer[1] + curr_layer[0] + prev_layer[prev_layer.len() - 1]
        );
        // 3 -> 2
        // 4 -> 4
        for i in 2..(2 * layer - 4) {
            set_or_return!(
                curr_layer[i],
                input,
                curr_layer[i - 1] + prev_layer[i - 2] + prev_layer[i - 1] + prev_layer[i]
            );
        }
        // Top-right corner and its neighbours
        set_or_return!(
            curr_layer[2 * layer - 4],
            input,
            curr_layer[2 * layer - 5]
                + prev_layer[2 * (layer - 1) - 3]
                + prev_layer[2 * (layer - 1) - 4]
        );
        set_or_return!(
            curr_layer[2 * layer - 3],
            input,
            curr_layer[2 * layer - 4] + prev_layer[2 * (layer - 1) - 3]
        );

        // Walking along the top edge now
        set_or_return!(
            curr_layer[2 * layer - 2],
            input,
            curr_layer[2 * layer - 3]
                + curr_layer[2 * layer - 4]
                + prev_layer[2 * (layer - 1) - 3]
                + prev_layer[2 * (layer - 1) - 2]
        );

        for i in 2..(2 * layer - 3) {
            set_or_return!(
                curr_layer[2 * layer - 3 + i],
                input,
                curr_layer[2 * layer - 4 + i]
                    + prev_layer[2 * (layer - 2) + i - 3]
                    + prev_layer[2 * (layer - 2) + i - 2]
                    + prev_layer[2 * (layer - 2) + i - 1]
            );
        }
        // The top-left corner, and its two surrounding squares
        set_or_return!(
            curr_layer[4 * (layer - 1) - 2],
            input,
            curr_layer[4 * (layer - 1) - 3]
                + prev_layer[4 * (layer - 2) - 2]
                + prev_layer[4 * (layer - 2) - 1]
        );
        set_or_return!(
            curr_layer[4 * (layer - 1) - 1],
            input,
            curr_layer[4 * (layer - 1) - 2] + prev_layer[4 * (layer - 2) - 1]
        );
        set_or_return!(
            curr_layer[4 * (layer - 1)],
            input,
            curr_layer[4 * (layer - 1) - 1]
                + curr_layer[4 * (layer - 1) - 2]
                + prev_layer[4 * (layer - 2) - 1]
                + prev_layer[4 * (layer - 2)]
        );

        // Walk along the left edge
        for i in 2..(2 * layer - 3) {
            set_or_return!(
                curr_layer[4 * (layer - 1) + i - 1],
                input,
                curr_layer[4 * (layer - 1) + i - 2]
                    + prev_layer[4 * (layer - 2) + i - 3]
                    + prev_layer[4 * (layer - 2) + i - 2]
                    + prev_layer[4 * (layer - 2) + i - 1]
            );
        }

        // The bottom-left corner, and its two surrounding squares
        set_or_return!(
            curr_layer[6 * layer - 8],
            input,
            curr_layer[6 * layer - 9] + prev_layer[6 * layer - 14] + prev_layer[6 * layer - 13]
        );
        set_or_return!(
            curr_layer[6 * layer - 7],
            input,
            curr_layer[6 * layer - 8] + prev_layer[6 * layer - 13]
        );
        set_or_return!(
            curr_layer[6 * layer - 6],
            input,
            curr_layer[6 * layer - 7]
                + curr_layer[6 * layer - 8]
                + prev_layer[6 * layer - 13]
                + prev_layer[6 * layer - 12]
        );

        // Walk along the bottom edge
        for i in 2..(2 * layer - 3) {
            set_or_return!(
                curr_layer[6 * (layer - 1) + i - 1],
                input,
                curr_layer[6 * (layer - 1) + i - 2]
                    + prev_layer[6 * (layer - 2) + i - 3]
                    + prev_layer[6 * (layer - 2) + i - 2]
                    + prev_layer[6 * (layer - 2) + i - 1]
            );
        }

        // The bottom-left corner and the square one to its left
        set_or_return!(
            curr_layer[8 * layer - 10],
            input,
            curr_layer[8 * layer - 11]
                + prev_layer[prev_layer.len() - 1]
                + prev_layer[prev_layer.len() - 2]
                + curr_layer[0]
        );
        set_or_return!(
            curr_layer[8 * layer - 9],
            input,
            curr_layer[8 * layer - 10] + prev_layer[prev_layer.len() - 1] + curr_layer[0]
        );

        prev_layer = curr_layer;
    }

    panic!("How could we have broken out of this infinite loop?!")
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(input));
    println!("part 2 => {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(1), 0);
        assert_eq!(part_1(2), 1);
        assert_eq!(part_1(3), 2);
        assert_eq!(part_1(4), 1);
        assert_eq!(part_1(5), 2);
        assert_eq!(part_1(6), 1);
        assert_eq!(part_1(7), 2);
        assert_eq!(part_1(8), 1);
        assert_eq!(part_1(9), 2);
        assert_eq!(part_1(10), 3);
        assert_eq!(part_1(11), 2);
        assert_eq!(part_1(12), 3);
        assert_eq!(part_1(13), 4);
        assert_eq!(part_1(14), 3);
        assert_eq!(part_1(15), 2);
        assert_eq!(part_1(16), 3);
        assert_eq!(part_1(17), 4);
        assert_eq!(part_1(18), 3);
        assert_eq!(part_1(19), 2);
        assert_eq!(part_1(20), 3);
        assert_eq!(part_1(21), 4);
        assert_eq!(part_1(22), 3);
        assert_eq!(part_1(23), 2);
        assert_eq!(part_1(24), 3);
        assert_eq!(part_1(25), 4);
        assert_eq!(part_1(26), 5);
        assert_eq!(part_1(1024), 31);
    }

    #[test]
    fn test_day_3() {
        let input = input();
        let answer = part_1(input);
        assert_eq!(answer, 438);
        let answer = part_2(input);
        assert_eq!(answer, 266330);
    }
}
