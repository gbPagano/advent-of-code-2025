advent_of_code_2025::gen_main!(part1, part2);

use fancy_regex::Regex;

fn part1(input: String) -> u64 {
    let re = Regex::new(r"^(\d+)\1$").unwrap();
    input
        .trim()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<u64>().unwrap(), y_str.parse::<u64>().unwrap()))
        .map(|(x, y)| {
            (x..=y)
                .into_iter()
                .map(|num| {
                    if re.is_match(&num.to_string()).unwrap() {
                        num
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

fn part2(input: String) -> u64 {
    let re = Regex::new(r"^(\d+)(\1)+$").unwrap();
    input
        .trim()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<u64>().unwrap(), y_str.parse::<u64>().unwrap()))
        .map(|(x, y)| {
            (x..=y)
                .into_iter()
                .map(|num| {
                    if re.is_match(&num.to_string()).unwrap() {
                        num
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}
