use day_6::day_6;

fn main() {
    let input = day_6::input();
    println!("part 1 => {}", day_6::part_1(&mut input.clone()));
    let mut input = input;
    println!("part 2 => {}", day_6::part_2(&mut input));
}
