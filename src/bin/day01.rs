advent_of_code_2025::gen_main!(part1, part2);

struct Dial {
    size: i32,
    pub curr: i32,
}

impl Dial {
    fn new(size: i32, curr: i32) -> Self {
        Self { size, curr }
    }

    fn rotate_left(&mut self, val: i32) {
        self.curr = (self.curr - val).rem_euclid(self.size + 1);
    }

    fn rotate_right(&mut self, val: i32) {
        self.curr = (self.curr + val).rem_euclid(self.size + 1);
    }
}

fn part1(input: String) -> i32 {
    let mut result = 0;
    let mut dial = Dial::new(99, 50);
    for line in input.lines() {
        match line.chars().nth(0) {
            Some('L') => {
                let n: i32 = line[1..].parse().unwrap();
                dial.rotate_left(n);
            }
            Some('R') => {
                let n: i32 = line[1..].parse().unwrap();
                dial.rotate_right(n);
            }
            _ => unreachable!(),
        }
        if dial.curr == 0 {
            result += 1;
        }
    }
    result
}

fn part2(input: String) {
    unimplemented!()
}
