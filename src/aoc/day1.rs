use std::collections::{BinaryHeap, HashMap, HashSet};

use super::utility;

pub fn solve() -> u32 {
    let input = utility::read("./src/input/day1.txt");
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    for p in input.iter() {
        let v = p
            .split_ascii_whitespace()
            .map(|p| -1 * p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        left.push(v[0]);
        right.push(v[1]);
    }
    let mut dist = 0;
    while let Some(l) = left.pop() {
        let r = right.pop().unwrap();
        dist += l.abs_diff(r);
    }
    dist
}

pub fn solve_2() -> i32 {
    let input = utility::read("./src/input/day1.txt");
    let mut left = BinaryHeap::new();
    let mut right = HashMap::new();
    for p in input.iter() {
        let v = p
            .split_ascii_whitespace()
            .map(|p| -1 * p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        left.push(v[0]);
        right.entry(v[1]).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut dist = 0;
    while let Some(l) = left.pop() {
        if let Some(&r) = right.get(&l) {
            dist += l.abs() * (r as i32).abs();
        }
    }
    dist
}
