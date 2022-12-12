use aoc_parse::{parser, prelude::*};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(u32, u32, u32, u32)> {
    let input = input.to_string() + "\n";
    let p = parser!(lines(u32 "-" u32 "," u32 "-" u32))
        .parse(&input)
        .unwrap();
    p
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<(u32, u32, u32, u32)>) -> u32 {
    input
        .iter()
        .map(
            |(a, b, c, d)| match (a <= c && b >= d) || (a >= c && b <= d) {
                true => 1,
                false => 0,
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day4::input_generator;

    use super::part1;

    #[test]
    pub fn totals_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let parsed = input_generator(input);
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    pub fn parses() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(
            input_generator(input),
            vec![
                (2, 4, 6, 8),
                (2, 3, 4, 5),
                (5, 7, 7, 9),
                (2, 8, 3, 7),
                (6, 6, 4, 6),
                (2, 6, 4, 8)
            ]
        )
    }
}
