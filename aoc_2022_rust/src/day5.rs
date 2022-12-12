use aoc_parse::{parser, prelude::*};

//     [M]             [Z]     [V]
//     [Z]     [P]     [L]     [Z] [J]
// [S] [D]     [W]     [W]     [H] [Q]
// [P] [V] [N] [D]     [P]     [C] [V]
// [H] [B] [J] [V] [B] [M]     [N] [P]
// [V] [F] [L] [Z] [C] [S] [P] [S] [G]
// [F] [J] [M] [G] [R] [R] [H] [R] [L]
// [G] [G] [G] [N] [V] [V] [T] [Q] [F]
// 1   2   3   4   5   6   7   8   9

// Hard-coding values, because I absolutely do not feel like parsing this rn lol.

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(u32, u32, u32)> {
    let input = input.to_string() + "\n";
    parser!(lines("move " u32 " from " u32 " to " u32))
        .parse(&input)
        .unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<(u32, u32, u32)>) -> String {
    let mut stack: Vec<Vec<char>> = vec![
        vec!['G', 'F', 'V', 'H', 'P', 'S'],
        vec!['G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M'],
        vec!['G', 'M', 'L', 'J', 'N'],
        vec!['N', 'G', 'Z', 'V', 'D', 'W', 'P'],
        vec!['V', 'R', 'C', 'B'],
        vec!['V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z'],
        vec!['T', 'H', 'P'],
        vec!['Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V'],
        vec!['F', 'L', 'G', 'P', 'V', 'Q', 'J'],
    ];

    for instruction in input {
        let (qty, from, to) = instruction;
        for _ in 0..*qty {
            if let Some(val) = stack[(from - 1) as usize].pop() {
                stack[(to - 1) as usize].push(val);
            }
        }
    }

    stack
        .iter()
        .filter_map(|s| s.last()).copied().collect()
}

#[cfg(test)]
mod tests {

    use crate::day5::input_generator;

    #[test]
    pub fn totals_part1() {
        let mut stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let input =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let parsed = input_generator(input);

        let ans: String = {
            for instruction in parsed {
                let (qty, from, to) = instruction;
                for _ in 0..qty {
                    if let Some(val) = stack[(from - 1) as usize].pop() {
                        stack[(to - 1) as usize].push(val);
                    }
                }
            }

            stack
                .iter()
                .filter_map(|s| s.last())
                .map(|val| *val)
                .collect()
        };
        assert_eq!(ans, "CMZ".to_string());
    }

    #[test]
    pub fn parses() {
        let input =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        assert_eq!(
            input_generator(input),
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]
        )
    }
}
