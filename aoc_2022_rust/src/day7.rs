#[derive(Debug)]
pub struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n")
        .map(|x| {
            x.to_string()
                .split_whitespace()
                .map(|y| y.to_string())
                .collect()
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<Vec<String>>) -> u32 {
    let mut root = Dir {
        name: "/".to_string(),
        files: Vec::new(),
        dirs: Vec::new(),
    };

    let mut current_dir: &Dir = &mut root;

    for i in 0..input.len() {
        for _j in 0..input[i].len() {
            if input[i][0].contains('$') {
                if !input[i][1].contains("cd") {
                    print!("1 {}", input[i][1]);
                }
                
            } else if input[i][0].contains("dir") {
                print!("2 {} ", input[i][1]);
            } else {
                print!("3 {} ", input[i][1]);
            }
        }
        print!("\n");
    }
    0
}

fn size_total(current: &Dir) -> u32 {
    if current.files.len() == 0 && !current.dirs.len() == 0 {
        return 0
    }
    let file_sum: u32 = current.files.iter().map(|x| x.size).sum();
    let dir_sum = {
        let mut totals: u32 = 0;
        for dir in &current.dirs {
            totals += size_total(&dir)
        }
        totals
    };
    file_sum + dir_sum

}

#[cfg(test)]
mod tests {

    use crate::day7::input_generator;

    use super::part1;

    #[test]
    pub fn totals_part1() {
        let input = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";
        let parsed = input_generator(input);
        assert_eq!(part1(&parsed), 95437);
    }

    #[test]
    pub fn parses() {
        let input = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";
        assert_eq!(
            input_generator(input),
            vec![
                vec!["$", "cd", "/"],
                vec!["$", "ls"],
                vec!["dir", "a"],
                vec!["14848514", "b.txt"],
                vec!["8504156", "c.dat"],
                vec!["dir", "d"],
                vec!["$", "cd", "a"],
                vec!["$", "ls"],
                vec!["dir", "e"],
                vec!["29116", "f"],
                vec!["2557", "g"],
                vec!["62596", "h.lst"],
                vec!["$", "cd", "e"],
                vec!["$", "ls"],
                vec!["584", "i"],
                vec!["$", "cd", ".."],
                vec!["$", "cd", ".."],
                vec!["$", "cd", "d"],
                vec!["$", "ls"],
                vec!["4060174", "j"],
                vec!["8033020", "d.log"],
                vec!["5626152", "d.ext"],
                vec!["7214296", "k"]
            ]
        )
    }
}
