pub mod day_10 {

    pub fn input_1() -> Vec<u8> {
        let input = include_str!("../input.txt");
        input
            .trim()
            .split(',')
            .map(|l| l.parse().unwrap())
            .collect::<Vec<u8>>()
    }

    pub fn input_2() -> Vec<u8> {
        let input = include_str!("../input.txt");
        input.trim().chars().map(|c| c as u8).collect::<Vec<u8>>()
    }

    fn rev<T>(start: usize, length: usize, b: &mut [T])
    where
        T: Copy,
    {
        for j in 0..(length / 2) {
            let tmp = b[(start + j) % b.len()];
            b[(start + j) % b.len()] = b[(start + length - j - 1) % b.len()];
            b[(start + length - j - 1) % b.len()] = tmp;
        }
    }

    pub(crate) struct HashState {
        pub(crate) v: Vec<u8>,
        pub(crate) curr_pos: usize,
        pub(crate) skip_size: usize,
    }

    pub(crate) fn new_state(size: usize) -> HashState {
        HashState {
            v: (0..size).map(|i| i as u8).collect(),
            curr_pos: 0,
            skip_size: 0,
        }
    }

    pub(crate) fn execute_round(state: &mut HashState, input: &[u8]) {
        for &i in input {
            let i = i as usize;
            rev(state.curr_pos, i, &mut state.v);
            state.curr_pos = (state.curr_pos + i + state.skip_size) % state.v.len();
            state.skip_size += 1;
        }
    }

    pub fn part_1(size: usize, input: &[u8]) -> u32 {
        let mut state = new_state(size);
        execute_round(&mut state, input);
        state.v[0] as u32 * state.v[1] as u32
    }

    fn densify(v: &[u8]) -> Vec<u8> {
        v.chunks_exact(16)
            .map(|i| i.iter().fold(0, |x, y| x ^ y))
            .collect()
    }

    // Convert a number from 0 to 15 into an ASCII hex char
    fn to_hex(i: u8) -> u8 {
        if i < 10 {
            i + b'0'
        } else {
            i - 10 + b'a'
        }
    }

    pub fn knot_hash_unsalted(bytes: &[u8]) -> String {
        let mut state = new_state(256);
        for _ in 0..64 {
            execute_round(&mut state, &bytes);
        }
        let dense = densify(&state.v);
        let mut answer = vec![0u8; 2 * dense.len()];
        for (i, b) in dense.iter().enumerate() {
            answer[2 * i] = to_hex(b / 16);
            answer[2 * i + 1] = to_hex(b % 16);
        }
        String::from_utf8(answer).unwrap()
    }

    pub fn knot_hash(bytes: &[u8]) -> String {
        let mut copy: Vec<u8> = bytes.to_vec();
        copy.extend(vec![17, 31, 73, 47, 23]);
        knot_hash_unsalted(&copy)
    }

    pub fn part_2(input: &[u8]) -> String {
        knot_hash(input)
    }
}

#[cfg(test)]
mod tests {
    use super::day_10::*;

    #[test]
    fn part1_known() {
        let mut state = new_state(5);
        execute_round(&mut state, &[3, 4, 1, 5]);
        assert_eq!(state.v, vec![3, 4, 2, 1, 0]);
        assert_eq!(state.skip_size, 4);
        assert_eq!(state.curr_pos, 4);
    }

    #[test]
    fn part2_known() {
        assert_eq!(knot_hash("".as_bytes()), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            knot_hash("AoC 2017".as_bytes()),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            knot_hash("1,2,3".as_bytes()),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            knot_hash("1,2,4".as_bytes()),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }

    #[test]
    fn test_day_10() {
        let input = input_1();
        assert_eq!(part_1(256, &input), 4114);
        let input = input_2();
        assert_eq!(part_2(&input), "2f8c3d2100fdd57cec130d928b0fd2dd");
    }
}
