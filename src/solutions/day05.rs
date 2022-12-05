const INPUT: &str = include_str!("../../inputs/day5");

pub fn run() -> (String, String) {
    (part1(), part2())
}

pub fn part1() -> String {
    let (mut stacks, procedures) = parse_input(INPUT);
    for p in procedures {
        for _ in 0..p.amount {
            let item = stacks[p.from - 1].pop().unwrap();
            stacks[p.to - 1].push(item);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}
pub fn part2() -> String {
    let (mut stacks, procedures) = parse_input(INPUT);
    for p in procedures {
        let mut tmp = vec![];
        for _ in 0..p.amount {
            let item = stacks[p.from - 1].pop().unwrap();
            tmp.push(item);
        }
        for _ in 0..p.amount {
            let item = tmp.pop().unwrap();
            stacks[p.to - 1].push(item);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Procedure>) {
    let (stack_input, procedure_input) = input.split_once("\n\n").unwrap();
    let stack_length = (stack_input.lines().map(|l| l.len()).max().unwrap() + 1) / 4;
    let mut stacks = vec![vec![]; stack_length];
    for line in stack_input
        .lines()
        .rev()
        .map(|l| l.as_bytes())
        .filter(|l| l.iter().any(|&c| c == b'['))
    {
        for stack_index in 0..stack_length {
            let c = *line.get(stack_index * 4 + 1).unwrap_or(&b'0');
            if c.is_ascii_alphabetic() {
                stacks[stack_index].push(c.into());
            }
        }
    }
    let procedures = procedure_input
        .lines()
        .map(|l| l.split_whitespace().filter_map(|s| s.parse::<usize>().ok()))
        .filter_map(|mut parts| {
            let amount = parts.next()?;
            let from = parts.next()?;
            let to = parts.next()?;
            Some(Procedure { amount, from, to })
        })
        .collect();
    (stacks, procedures)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_parse_input() {
        let (stacks, procedures) = parse_input(TEST_INPUT);

        assert_eq!(stacks[0][0], 'Z');
        assert_eq!(stacks[1][2], 'D');
        assert_eq!(stacks[2][0], 'P');

        assert_eq!(procedures[0].from, 2);
        assert_eq!(procedures[1].amount, 3);
        assert_eq!(procedures[2].to, 1);
    }
}
