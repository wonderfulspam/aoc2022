const INPUT: &str = include_str!("../../inputs/day4");

pub fn run() -> (String, String) {
    (part1().to_string(), part2().to_string())
}

pub fn part1() -> usize {
    let pairs = get_assignment_pairs();
    pairs
        .filter(|(s1, e1, s2, e2)| s1 >= s2 && e1 <= e2 || s2 >= s1 && e2 <= e1)
        .count()
}

pub fn part2() -> usize {
    let pairs = get_assignment_pairs();
    pairs
        .filter(|(s1, e1, s2, e2)| s2 <= e1 && s1 <= e2)
        .count()
}
fn get_assignment_pairs() -> impl Iterator<Item = (usize, usize, usize, usize)> {
    INPUT.lines().filter_map(|l| {
        let (assign_one, assign_two) = l.split_once(',')?;
        let (start_one, end_one) = assign_one.split_once('-')?;
        let (start_two, end_two) = assign_two.split_once('-')?;
        Some((
            start_one.parse().ok()?,
            end_one.parse().ok()?,
            start_two.parse().ok()?,
            end_two.parse().ok()?,
        ))
    })
}
