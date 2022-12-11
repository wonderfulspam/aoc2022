#[allow(dead_code)]
const INPUT: &str = include_str!("../../inputs/day6");

use itertools::Itertools;
pub fn run() -> (String, String) {
    (part1().to_string(), part2().to_string())
}

pub fn part1() -> usize {
    get_position_first_n_unequal(INPUT, 4)
}
pub fn part2() -> usize {
    get_position_first_n_unequal(INPUT, 14)
}

fn get_position_first_n_unequal(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(|slice| slice.iter().tuple_combinations().all(|(a, b)| a != b))
        .unwrap()
        + size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let test_input: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let index = get_position_first_n_unequal(test_input, 4);
        assert_eq!(index, 7);
    }

    #[test]
    fn test_part1_b() {
        let test_input: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = get_position_first_n_unequal(test_input, 4);
        assert_eq!(index, 6);
    }
}
