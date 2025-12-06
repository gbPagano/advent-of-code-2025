advent_of_code_2025::gen_main!(part1, part2);

#[derive(Debug)]
struct Dial {
    size: i32,
    pub curr: i32,
    pub zero_stopped: i32,
    pub zero_passes: i32,
}

impl Dial {
    fn new(size: i32, curr: i32) -> Self {
        Self {
            size,
            curr,
            zero_stopped: 0,
            zero_passes: 0,
        }
    }

    fn rotate_left(&mut self, val: i32) {
        let (div, mod_) = (val / self.size, val % self.size);
        self.zero_passes += div;
        if self.curr - mod_ <= 0 && self.curr != 0 {
            self.zero_passes += 1
        }

        self.curr = (self.curr - val).rem_euclid(self.size);
        if self.curr == 0 {
            self.zero_stopped += 1
        }
    }

    fn rotate_right(&mut self, val: i32) {
        let (div, mod_) = (val / self.size, val % self.size);
        self.zero_passes += div;
        if self.curr + mod_ >= self.size {
            self.zero_passes += 1
        }

        self.curr = (self.curr + val).rem_euclid(self.size);
        if self.curr == 0 {
            self.zero_stopped += 1
        }
    }
}

fn part1(input: String) -> i32 {
    let mut dial = Dial::new(100, 50);
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
    }
    dial.zero_stopped
}

fn part2(input: String) -> i32 {
    let mut dial = Dial::new(100, 50);
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
    }
    dial.zero_passes
}
