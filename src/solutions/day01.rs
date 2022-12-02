use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day1");

pub fn run() -> (String, String) {
    (part1().to_string(), part2().to_string())
}

pub fn part1() -> u32 {
    let mut elf_counts = get_elf_calorie_counts();
    elf_counts.pop().unwrap()
}

pub fn part2() -> u32 {
    let mut elf_counts = get_elf_calorie_counts();
    elf_counts.pop().unwrap() + elf_counts.pop().unwrap() + elf_counts.pop().unwrap()
}

fn get_elf_calorie_counts() -> Vec<u32> {
    INPUT
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .collect()
}
