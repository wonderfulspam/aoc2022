use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day3");

pub fn run() -> (String, String) {
    (part1().to_string(), part2().to_string())
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| (&l[..l.len() / 2], &l[l.len() / 2..]))
        .map(|(a, b)| same_bytes(a, b))
        .map(|v| byte_to_value(v[0]))
        .sum()
}

fn same_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().copied().filter(|c| b.contains(c)).collect()
}

fn byte_to_value(c: u8) -> usize {
    match c {
        b'a'..=b'z' => c as usize - b'a' as usize + 1,
        b'A'..=b'Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| l.as_bytes())
        .tuples()
        .map(|(a, b, c)| same_bytes(a, &same_bytes(b, c)))
        .map(|v| byte_to_value(v[0]))
        .sum()
}
