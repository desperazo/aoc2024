use std::ops::Index;

use super::utility;

pub fn solve() -> u32 {
    let input = utility::read("./src/input/day3.txt");
    let mut ans = 0;
    let words = vec!['m', 'u', 'l', '(', 'X', 'X'];
    for v in input.iter() {
        let mut wc = 0;
        let mut left = String::new();
        let mut right = String::new();
        for c in v.chars() {
            if wc >= words.len() {
                left.clear();
                right.clear();
                wc = 0;
            }
            if c == words[wc] {
                wc += 1;
            } else if words[wc] == 'X' {
                if c.is_digit(10) {
                    if wc == 5 {
                        right.push(c);
                    } else {
                        left.push(c);
                    }
                } else if wc == 4 && c == ',' {
                    wc += 1;
                } else if wc == 5 && c == ')' {
                    ans += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
                    wc = usize::MAX;
                } else {
                    wc = usize::MAX;
                }
            } else {
                wc = usize::MAX;
            }
        }
    }
    ans
}

pub fn solve_2() -> u32 {
    let input = utility::read("./src/input/day3.txt");
    let mut ans = 0;
    let words = vec!['m', 'u', 'l', '(', 'X', 'X'];
    let mut enable = true;
    for v in input.iter() {
        let mut wc = 0;
        let mut left = String::new();
        let mut right = String::new();
        for (i, c) in v.chars().enumerate() {
            if c == 'd' && i < v.len() - 7 && &v[i..i + 7] == "don't()" {
                enable = false;
                continue;
            }
            if c == 'd' && i < v.len() - 4 && &v[i..i + 4] == "do()" {
                enable = true;
            }
            if !enable {
                continue;
            }
            if wc >= words.len() {
                left.clear();
                right.clear();
                wc = 0;
            }
            if c == words[wc] {
                wc += 1;
            } else if words[wc] == 'X' {
                if c.is_digit(10) {
                    if wc == 5 {
                        right.push(c);
                    } else {
                        left.push(c);
                    }
                } else if wc == 4 && c == ',' {
                    wc += 1;
                } else if wc == 5 && c == ')' {
                    ans += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
                    wc = usize::MAX;
                } else {
                    wc = usize::MAX;
                }
            } else {
                wc = usize::MAX;
            }
        }
    }
    ans
}
