use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day1");

fn main() {
    let mut elf_counts: Vec<_> = INPUT
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .collect();
    let max = elf_counts.pop().unwrap();
    dbg!(max);
    let max_three = max + elf_counts.pop().unwrap() + elf_counts.pop().unwrap();
    dbg!(max_three);
}
