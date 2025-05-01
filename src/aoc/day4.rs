use std::vec;

use super::utility;

pub fn solve() -> u32 {
    let dat = parse();
    let size = dat.len();
    let mut ans = 0;
    for i in 0..size {
        for j in 0..size {
            // x-axis
            if size - j >= 4
                && dat[i][j] == 'X'
                && dat[i][j + 1] == 'M'
                && dat[i][j + 2] == 'A'
                && dat[i][j + 3] == 'S'
            {
                ans += 1;
            }
            if j >= 3
                && dat[i][j] == 'X'
                && dat[i][j - 1] == 'M'
                && dat[i][j - 2] == 'A'
                && dat[i][j - 3] == 'S'
            {
                ans += 1
            }

            // y-axis
            if size - i >= 4
                && dat[i][j] == 'X'
                && dat[i + 1][j] == 'M'
                && dat[i + 2][j] == 'A'
                && dat[i + 3][j] == 'S'
            {
                ans += 1;
            }
            if i >= 3
                && dat[i][j] == 'X'
                && dat[i - 1][j] == 'M'
                && dat[i - 2][j] == 'A'
                && dat[i - 3][j] == 'S'
            {
                ans += 1
            }

            // diagx-axis
            if i >= 3
                && size - j >= 4
                && dat[i][j] == 'X'
                && dat[i - 1][j + 1] == 'M'
                && dat[i - 2][j + 2] == 'A'
                && dat[i - 3][j + 3] == 'S'
            {
                ans += 1;
            }

            if size - i >= 4
                && size - j >= 4
                && dat[i][j] == 'X'
                && dat[i + 1][j + 1] == 'M'
                && dat[i + 2][j + 2] == 'A'
                && dat[i + 3][j + 3] == 'S'
            {
                ans += 1;
            }

            // diagy-axis
            if size - i >= 4
                && j >= 3
                && dat[i][j] == 'X'
                && dat[i + 1][j - 1] == 'M'
                && dat[i + 2][j - 2] == 'A'
                && dat[i + 3][j - 3] == 'S'
            {
                ans += 1
            }

            if i >= 3
                && j >= 3
                && dat[i][j] == 'X'
                && dat[i - 1][j - 1] == 'M'
                && dat[i - 2][j - 2] == 'A'
                && dat[i - 3][j - 3] == 'S'
            {
                ans += 1
            }
        }
    }
    ans
}

pub fn solve_2() -> u32 {
    let dat = parse();
    let size = dat.len();
    let mut ans = 0;
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let x = dat[i][j];
            let q1 = dat[i - 1][j + 1];
            let q2 = dat[i + 1][j + 1];
            let q3 = dat[i + 1][j - 1];
            let q4 = dat[i - 1][j - 1];
            if x != 'A' {
                continue;
            }
            if (q1 == 'M' && q3 == 'S' || q1 == 'S' && q3 == 'M')
                && (q2 == 'M' && q4 == 'S' || q2 == 'S' && q4 == 'M')
            {
                ans += 1;
            }
        }
    }
    ans
}

fn parse() -> Vec<Vec<char>> {
    let mut dat = vec![];
    let input = utility::read("./src/input/day4.txt");
    for v in input.iter() {
        dat.push(v.chars().collect());
    }
    dat
}
