//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given a non-negative integer x, compute and return the square root of x. 
//
// Since the return type is an integer, the decimal digits are truncated, and 
//only the integer part of the result is returned. 
//
// Note: You are not allowed to use any built-in exponent function or operator, 
//such as pow(x, 0.5) or x ** 0.5. 
//
// 
// Example 1: 
//
// 
//Input: x = 4
//Output: 2
// 
//
// Example 2: 
//
// 
//Input: x = 8
//Output: 2
//Explanation: The square root of 8 is 2.82842..., and since the decimal part 
//is truncated, 2 is returned. 
//
// 
// Constraints: 
//
// 
// 0 <= x <= 2³¹ - 1 
// 
// Related Topics Math Binary Search 👍 2464 👎 2597


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u64;
        let mut nx = x;
        while nx * nx > x  {
            nx = ((nx + (x / nx)) / 2);
        }
        nx as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::my_sqrt(4);
        assert_eq!(ret, 2);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::my_sqrt(8);
        assert_eq!(ret, 2);
    }
    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::my_sqrt(15);
        assert_eq!(ret, 3);
    }
    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::my_sqrt(2147395599);
        assert_eq!(ret, 46339);
    }
    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::my_sqrt(2147483647);
        assert_eq!(ret, 46340);
    }
}