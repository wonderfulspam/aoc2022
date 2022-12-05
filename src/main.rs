fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Please provide an argument")
        .parse()
        .expect("Please provide a valid number");
    let (part1, part2) = match day {
        1 => aoc2022::solutions::day01::run(),
        2 => aoc2022::solutions::day02::run(),
        3 => aoc2022::solutions::day03::run(),
        4 => aoc2022::solutions::day04::run(),
        _ => unimplemented!("Not done yet"),
    };
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
