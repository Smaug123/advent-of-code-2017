pub mod day_17 {

    pub fn input() -> usize {
        let input = include_str!("../input.txt");
        input.trim().parse().unwrap()
    }

    struct CircularBuffer<T> {
        elts: Vec<T>,
        current_size: usize,
    }

    fn new_buffer<T>(size: usize) -> CircularBuffer<T>
    where
        T: Default + Clone,
    {
        CircularBuffer {
            elts: vec![Default::default(); size],
            current_size: 0,
        }
    }

    fn walk<T>(current_pos: usize, i: usize, buf: &CircularBuffer<T>) -> usize {
        (i + current_pos) % buf.current_size
    }

    fn element_at<T>(pos: usize, buf: &CircularBuffer<T>) -> &T {
        &buf.elts[pos % buf.current_size]
    }

    fn insert_after<T>(pos: usize, i: T, buf: &mut CircularBuffer<T>)
    where
        T: Copy,
    {
        for j in (pos + 2..=buf.current_size).rev() {
            buf.elts[j] = buf.elts[j - 1];
        }
        buf.elts[pos + 1] = i;
        buf.current_size += 1;
    }

    // Returns the position of the write head, and the buffer.
    fn proceed(step_size: usize, count: usize) -> (usize, CircularBuffer<u32>) {
        let mut buffer = new_buffer::<u32>(count + 1);
        insert_after(0, 0, &mut buffer);
        let mut pos = 0;
        for i in 1..=count {
            // + 1 because we need to pick up the inserted element too
            pos = walk(pos + 1, step_size, &buffer);
            insert_after(pos, i as u32, &mut buffer);
        }
        (pos + 1, buffer)
    }

    pub fn part_1(step_size: usize) -> u32 {
        let (head, buffer) = proceed(step_size, 2017);
        *element_at(head + 1, &buffer)
    }

    pub fn part_2(step_size: usize, bound: usize) -> u32 {
        let mut after_zero = 1u32;
        let mut current_head: usize = 1;
        for i in 2usize..=bound {
            let insert_after = (current_head + step_size) % i;
            if insert_after == 0 {
                after_zero = i as u32;
            }
            current_head = (insert_after + 1) % (i + 1);
        }

        after_zero
    }
}

#[cfg(test)]
mod tests {
    use super::day_17::*;

    #[test]
    fn part1_known() {
        assert_eq!(part_1(3), 638);
    }

    #[test]
    fn part2_known() {
        assert_eq!(part_2(3, 2), 2);
        assert_eq!(part_2(3, 3), 2);
        assert_eq!(part_2(3, 4), 2);
        assert_eq!(part_2(3, 5), 5);
        assert_eq!(part_2(3, 6), 5);
        assert_eq!(part_2(3, 7), 5);
        assert_eq!(part_2(3, 8), 5);
        assert_eq!(part_2(3, 9), 9);
    }

    #[test]
    fn test_day_17() {
        let input = input();
        assert_eq!(part_1(input), 1173);
        assert_eq!(part_2(input, 50000000), 1930815);
    }
}
