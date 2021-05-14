use day_10::day_10;

fn main() {
    let input = day_10::input_1();
    println!("part 1 => {}", day_10::part_1(256, &input));

    let mut input = day_10::input_2();
    input.extend(vec![17, 31, 73, 47, 23]);
    println!("part 2 => {}", day_10::part_2(&input));
}
