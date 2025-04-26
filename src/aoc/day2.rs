use super::utility;

pub fn solve() -> i32 {
    let data = parse();
    let mut ans = 0;
    for v in data.iter() {
        let mut count = 0;
        for i in 1..v.len() {
            let dif = (v[i - 1] - v[i]).abs();
            if dif < 1 || dif > 3 {
                break;
            }
            if (v[0] < v[1]) != (v[i - 1] < v[i]) {
                break;
            }
            count += 1;
        }
        if count == v.len() - 1 {
            ans += 1;
        }
    }
    ans
}

pub fn solve_2() -> i32 {
    let data = parse();
    let mut ans = 0;
    for v in data.iter() {
        let (mut inc, mut dec, mut inc_prev, mut dec_prev) = (0, 0, 0, 0);
        for i in 1..v.len() {
            let inc_d = (v[inc_prev] - v[i]).abs();
            if inc_d <= 3 && v[inc_prev] < v[i] {
                inc_prev = i;
                inc += 1;
            } else if inc_prev > 0 && v[inc_prev - 1] < v[i] {
                inc_prev = i;
            } else if inc_prev == 0 {
                inc_prev = 1;
            }

            let dec_d = (v[dec_prev] - v[i]).abs();
            if dec_d <= 3 && v[dec_prev] > v[i] {
                dec_prev = i;
                dec += 1;
            } else if dec_prev > 0 && v[dec_prev - 1] > v[i] {
                dec_prev = i;
            } else if dec_prev == 0 {
                dec_prev = 1;
            }
        }
        let expected = v.len() - 2;
        if inc >= expected || dec >= expected {
            ans += 1;
        }
    }
    ans
}

fn parse() -> Vec<Vec<i32>> {
    let input = utility::read("./src/input/day2.txt");
    let data = input
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    data
}
