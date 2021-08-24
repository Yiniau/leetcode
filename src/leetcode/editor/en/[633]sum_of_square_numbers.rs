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
        let mut r = (c as f64).sqrt() as i32;
        while r >= l {
            let t = l.pow(2) + r.pow(2);
            if c == t {
                return true;
            }
            if t > c {
                r -= 1;
            } else {
                l += 1;
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