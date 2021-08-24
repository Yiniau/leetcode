//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given a non-negative integer c, decide whether there're two integers a and b 
//such that a² + b² = c. 
//
// 
// Example 1: 
//
// 
//Input: c = 5
//Output: true
//Explanation: 1 * 1 + 2 * 2 = 5
// 
//
// Example 2: 
//
// 
//Input: c = 3
//Output: false
// 
//
// Example 3: 
//
// 
//Input: c = 4
//Output: true
// 
//
// Example 4: 
//
// 
//Input: c = 2
//Output: true
// 
//
// Example 5: 
//
// 
//Input: c = 1
//Output: true
// 
//
// 
// Constraints: 
//
// 
// 0 <= c <= 2³¹ - 1 
// 
// Related Topics Math Two Pointers Binary Search 👍 793 👎 400


//leetcode submit region begin(Prohibit modification and deletion)
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
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use crate::_sum_of_square_numbers::Solution;

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