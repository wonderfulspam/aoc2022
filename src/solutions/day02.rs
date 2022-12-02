use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day2");

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();

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

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = ();

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
    part1_inner(INPUT)
}

// Inner function to allow passing test input
fn part1_inner(input: &str) -> u32 {
    input
        .lines()
        .map(choices_from_line)
        .fold(0, |acc, (my_choice, elf_choice)| {
            let outcome = get_outcome(&my_choice, &elf_choice);
            acc + get_score(&my_choice, &outcome)
        })
}

fn choices_from_line(line: &str) -> (Choice, Choice) {
    let (elf_choice, my_choice) = line.split_once(' ').unwrap();
    let my_choice = Choice::from_str(my_choice).unwrap();
    let elf_choice = Choice::from_str(elf_choice).unwrap();
    (my_choice, elf_choice)
}

pub fn part2() -> u32 {
    part2_inner(INPUT)
}

// Inner function to allow passing test input
fn part2_inner(input: &str) -> u32 {
    input
        .lines()
        .map(choice_and_outcome_from_line)
        .fold(0, |acc, (my_choice, outcome)| {
            acc + get_score(&my_choice, &outcome)
        })
}

fn choice_and_outcome_from_line(line: &str) -> (Choice, Outcome) {
    let (elf_choice, outcome) = line.split_once(' ').unwrap();
    let elf_choice = Choice::from_str(elf_choice).unwrap();
    let outcome = Outcome::from_str(outcome).unwrap();
    let my_choice = Choice::from_outcome(&outcome, &elf_choice);
    (my_choice, outcome)
}

pub fn run() -> (String, String) {
    let part1 = part1();
    let part2 = part2();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "A Y\nB X\nC Z";
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
    fn test_parse_choices() {
        let line = "A Y";
        let (my_choice, elf_choice) = choices_from_line(line);
        assert_eq!(elf_choice, Choice::Rock);
        assert_eq!(my_choice, Choice::Paper);
    }

    #[test]
    fn test_parse_choice_and_outcome() {
        let line = "A Y";
        let (elf_choice, outcome) = choice_and_outcome_from_line(line);
        assert_eq!(elf_choice, Choice::Rock);
        assert_eq!(outcome, Outcome::Draw);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1_inner(TEST_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_inner(TEST_INPUT), 12);
    }
}
