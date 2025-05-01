pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::aoc::{day1, day2, day3, day4};

    #[test]
    pub fn day1() {
        let ans = day1::solve();
        assert_eq!(1341714, ans);
    }

    #[test]
    pub fn day1_2() {
        let ans = day1::solve_2();
        assert_eq!(27384707, ans);
    }

    #[test]
    pub fn day2() {
        let ans = day2::solve();
        assert_eq!(490, ans);
    }

    #[test]
    pub fn day2_2() {
        let ans = day2::solve_2();
        assert_eq!(536, ans);
    }

    #[test]
    pub fn day3() {
        let ans = day3::solve();
        assert_eq!(183669043, ans);
    }

    #[test]
    pub fn day3_2() {
        let ans = day3::solve_2();
        assert_eq!(59097164, ans);
    }

    #[test]
    pub fn day4() {
        let ans = day4::solve();
        assert_eq!(2406, ans);
    }

    #[test]
    pub fn day4_2() {
        let ans = day4::solve_2();
        assert_eq!(1807, ans);
    }
}
