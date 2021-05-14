use day_2::day_2;

fn main() {
    let input = day_2::input();
    println!(
        "part 1 => {}",
        day_2::part_1(&mut input.iter().map(|r| r.iter().cloned()))
    );
    println!(
        "part 2 => {}",
        day_2::part_2(&mut input.iter().map(|r| r.iter().cloned()))
    );
}
