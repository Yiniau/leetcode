#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode id=633 lang=rust
 *
 * [633] Sum of Square Numbers
 */

// @lc code=start
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        if c == 0 {
            return true;
        }
        let mut l: i32 = 0;
        let mut l_powed = l.pow(2);
        while c - l_powed > 0 {
            let r_powed = c - l_powed;
            let r = (r_powed as f64).sqrt();
            if r.is_nan() || r.fract() != 0.0 {
                l += 1;
                l_powed = l.pow(2);
            } else {
                return true;
            }
        }
        false
    }
}
// @lc code=end


#[cfg(test)]
mod tests {
    use crate::sum_of_square_numbers::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::judge_square_sum(
            5
        );
        assert_eq!(ret, true);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::judge_square_sum(
            3
        );
        assert_eq!(ret, false);
    }
    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::judge_square_sum(
            4
        );
        assert_eq!(ret, true);
    }
    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::judge_square_sum(
            2
        );
        assert_eq!(ret, true);
    }
    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::judge_square_sum(
            1
        );
        assert_eq!(ret, true);
    }
    #[test]
    fn test_6() {
        // test code here
        let ret = Solution::judge_square_sum(
            0
        );
        assert_eq!(ret, true);
    }
}