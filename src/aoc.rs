pub mod day1;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::day1;

    #[test]
    pub fn day1() {
        let ans = day1::solve();
        assert_eq!(1341714, ans);
    }

    #[test]
    pub fn day1_2() {
        let ans = day1::solve_2();
        assert_eq!(1341714, ans);
    }
}
