use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    parser!(alpha*).parse(input).unwrap()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<char>) -> u32 {
    for i in 0..input.len() {
        if i >= 3 {
            let mut checked: HashSet<char> = HashSet::new();
            checked.insert(input[i - 3]);
            checked.insert(input[i - 2]);
            checked.insert(input[i - 1]);
            checked.insert(input[i]);

            if checked.len() == 4 {
                return i as u32 + 1;
            }
        }
    }
    0
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<char>) -> u32 {
    let mut ans = 0;
    for i in 0..input.len() {
        if i >= 15 {
            let mut checked: HashSet<char> = HashSet::new();
            checked.insert(input[i - 13]);
            checked.insert(input[i - 12]);
            checked.insert(input[i - 11]);
            checked.insert(input[i - 10]);
            checked.insert(input[i - 9]);
            checked.insert(input[i - 8]);
            checked.insert(input[i - 7]);
            checked.insert(input[i - 6]);
            checked.insert(input[i - 5]);
            checked.insert(input[i - 4]);
            checked.insert(input[i - 3]);
            checked.insert(input[i - 2]);
            checked.insert(input[i - 1]);
            checked.insert(input[i]);

            if checked.len() == 14 {
                ans = i + 1;
                break;
            }
        }
    }
    ans as u32
}

#[cfg(test)]
mod tests {

    use crate::day6::input_generator;

    use super::{part1, part2};

    #[test]
    pub fn totals_part1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let parsed = input_generator(input);
        assert_eq!(part1(&parsed), 7);
    }

    #[test]
    pub fn totals_part2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let parsed = input_generator(input);
        assert_eq!(part2(&parsed), 19);
    }

    #[test]
    pub fn parses() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(
            input_generator(input),
            vec![
                'm', 'j', 'q', 'j', 'p', 'q', 'm', 'g', 'b', 'l', 'j', 's', 'p', 'h', 'd', 'z',
                't', 'n', 'v', 'j', 'f', 'q', 'w', 'r', 'c', 'g', 's', 'm', 'l', 'b'
            ]
        );
    }
}
