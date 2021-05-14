use day_5::day_5;

fn main() {
    let input = day_5::input();
    println!("part 1 => {}", day_5::part_1(&mut input.clone()));
    let mut input = input;
    println!("part 2 => {}", day_5::part_2(&mut input));
}
