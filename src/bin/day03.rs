advent_of_code_2025::gen_main!(part1, part2);

fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let bank: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let mut max_idx = bank.len() - 2;
            let mut min_idx = bank.len() - 1;
            for idx in (0..bank.len() - 2).rev() {
                if bank[idx] >= bank[max_idx] {
                    // when min_idx is last he can be bigger
                    if bank[max_idx] > bank[min_idx] {
                        min_idx = max_idx;
                    }
                    max_idx = idx;
                }
            }
            format!("{}{}", bank[max_idx], bank[min_idx])
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn part2(input: String) {
    unimplemented!()
}
