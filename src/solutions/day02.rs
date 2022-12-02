use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day2");

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice = match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => unreachable!(),
        };
        Ok(choice)
    }
}

impl Choice {
    fn value(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn from_outcome(outcome: &Outcome, elf_choice: &Choice) -> Choice {
        match (outcome, elf_choice) {
            (Outcome::Win, Choice::Rock) => Choice::Paper,
            (Outcome::Win, Choice::Paper) => Choice::Scissors,
            (Outcome::Win, Choice::Scissors) => Choice::Rock,
            (Outcome::Draw, Choice::Rock) => Choice::Rock,
            (Outcome::Draw, Choice::Paper) => Choice::Paper,
            (Outcome::Draw, Choice::Scissors) => Choice::Scissors,
            (Outcome::Loss, Choice::Rock) => Choice::Scissors,
            (Outcome::Loss, Choice::Paper) => Choice::Rock,
            (Outcome::Loss, Choice::Scissors) => Choice::Paper,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        };
        Ok(outcome)
    }
}
impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn get_outcome(my_choice: &Choice, elf_choice: &Choice) -> Outcome {
    use Outcome::*;
    match (my_choice, elf_choice) {
        (Choice::Rock, Choice::Rock) => Draw,
        (Choice::Rock, Choice::Paper) => Loss,
        (Choice::Rock, Choice::Scissors) => Win,
        (Choice::Paper, Choice::Rock) => Win,
        (Choice::Paper, Choice::Paper) => Draw,
        (Choice::Paper, Choice::Scissors) => Loss,
        (Choice::Scissors, Choice::Rock) => Loss,
        (Choice::Scissors, Choice::Paper) => Win,
        (Choice::Scissors, Choice::Scissors) => Draw,
    }
}

fn get_score(my_choice: &Choice, outcome: &Outcome) -> u32 {
    my_choice.value() + outcome.value()
}

pub fn part1() -> u32 {
    _part1(INPUT)
}

// Inner function to allow passing test input
fn _part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (elf_choice, my_choice) = l.split_once(' ').unwrap();
            let my_choice = Choice::from_str(my_choice).unwrap();
            let elf_choice = Choice::from_str(elf_choice).unwrap();
            (my_choice, elf_choice)
        })
        .fold(0, |acc, (my_choice, elf_choice)| {
            let outcome = get_outcome(&my_choice, &elf_choice);
            acc + get_score(&my_choice, &outcome)
        })
}

pub fn part2() -> u32 {
    _part2(INPUT)
}

// Inner function to allow passing test input
fn _part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (elf_choice, outcome) = l.split_once(' ').unwrap();
            let elf_choice = Choice::from_str(elf_choice).unwrap();
            let outcome = Outcome::from_str(outcome).unwrap();
            let my_choice = Choice::from_outcome(&outcome, &elf_choice);
            (my_choice, outcome)
        })
        .fold(0, |acc, (my_choice, outcome)| {
            acc + get_score(&my_choice, &outcome)
        })
}

pub fn run() -> (String, String) {
    let part1 = part1();
    let part2 = part2();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_outcome() {
        let my_choice = Choice::Paper;
        let elf_choice = Choice::Rock;

        let outcome = get_outcome(&my_choice, &elf_choice);
        let points = my_choice.value() + outcome.value();
        assert_eq!(points, 8);
    }

    #[test]
    fn test_part1() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(_part1(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(_part2(input), 12);
    }
}
