use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("#{id} @ {left},{top}: {width}x{height}")]
pub struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn right(&self) -> usize {
        self.left + self.width
    }
    fn bottom(&self) -> usize {
        self.top + self.height
    }
    fn intersects(&self, other: &Self) -> bool {
        let intersects_x = self.right() >= other.left && self.left <= other.right();
        let intersects_y = self.bottom() >= other.top && self.top <= other.bottom();
        intersects_x && intersects_y
    }
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|l| l.parse().expect("Bad line"))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> usize {
    let mut grid = [[0 as u8; 1000]; 1000];
    let mut count = 0;
    for c in input {
        for x in c.left..(c.left + c.width) {
            for y in c.top..(c.top + c.height) {
                if grid[x][y] == 1 {
                    count += 1;
                }
                grid[x][y] += 1;
            }
        }
    }
    return count;
}

use std::collections::HashSet;

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> usize {
    let mut candidates: HashSet<usize> = input.iter().map(|c| c.id).collect();
    for c1ix in 0..input.len() {
        for c2ix in c1ix + 1..input.len() {
            let c1 = &input[c1ix];
            let c2 = &input[c2ix];
            if c1.intersects(c2) {
                candidates.remove(&c1.id);
                candidates.remove(&c2.id);
            }
        }
    }
    *candidates.iter().nth(0).expect("Not found")
}

#[cfg(test)]
const HINT_INPUT: &'static str = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;

#[test]
pub fn test_example() {
    assert_eq!(part1(&gen(HINT_INPUT.trim())), 4);
}
