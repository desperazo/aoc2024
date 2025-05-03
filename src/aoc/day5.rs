use std::collections::HashSet;

use super::utility;

pub fn solve() -> i32 {
    let (set, input) = parse();
    let mut sum = 0;
    'outer: for v in input.iter() {
        for w in v.windows(2) {
            let text = format!("{}|{}", w[1], w[0]);
            if set.contains(&text) {
                continue 'outer;
            }
        }
        sum += v[v.len() / 2];
    }
    sum
}

pub fn solve_2() -> i32 {
    let (set, mut input) = parse();
    let mut sum = 0;
    for v in input.iter_mut() {
        let mut swap = false;
        let mut i = 1;
        while i < v.len() {
            let text = format!("{}|{}", v[i], v[i - 1]);
            if !set.contains(&text) {
                i += 1;
                continue;
            }
            let left = v.iter().position(|x| *x == v[i - 1]).unwrap();
            let right = v.iter().position(|x| *x == v[i]).unwrap();
            v.swap(left, right);
            swap = true;
            i = 1;
        }
        if swap {
            sum += v[v.len() / 2];
        }
    }
    sum
}

fn parse() -> (HashSet<String>, Vec<Vec<i32>>) {
    let mut set = HashSet::new();
    let mut dat = vec![];
    let input = utility::read("./src/input/day5.txt");
    for v in input.iter() {
        if v.len() != 5 {
            dat.push(v.split(',').map(|x| x.parse::<i32>().unwrap()).collect());
        } else {
            set.insert(v.to_string());
        }
    }
    (set, dat)
}
