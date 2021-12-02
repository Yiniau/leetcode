//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an array nums with n objects colored red, white, or blue, sort them in-
//place so that objects of the same color are adjacent, with the colors in the 
//order red, white, and blue. 
//
// We will use the integers 0, 1, and 2 to represent the color red, white, and 
//blue, respectively. 
//
// You must solve this problem without using the library's sort function. 
//
// 
// Example 1: 
// Input: nums = [2,0,2,1,1,0]
//Output: [0,0,1,1,2,2]
// Example 2: 
// Input: nums = [2,0,1]
//Output: [0,1,2]
// Example 3: 
// Input: nums = [0]
//Output: [0]
// Example 4: 
// Input: nums = [1]
//Output: [1]
// 
// 
// Constraints: 
//
// 
// n == nums.length 
// 1 <= n <= 300 
// nums[i] is 0, 1, or 2. 
// 
//
// 
// Follow up: Could you come up with a one-pass algorithm using only constant 
//extra space? 
// Related Topics Array Two Pointers Sorting 👍 7788 👎 360


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut p0 = 0;
        let mut p1 = 0;
        let mut p2 = nums.len() - 1;

        while p1 <= p2 {
            let t = nums[p1];
            if t == 0 {
                nums.swap(p0, p1);
                p0 += 1;
                p1 += 1;
            } else if t == 1 {
                p1 += 1;
            } else if t == 2 {
                nums.swap(p1, p2);
                if p2 > 0 {
                    p2 -= 1;
                } else {
                    break;
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let mut arr = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        // test code here
        let mut arr = vec![2, 0, 1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        // test code here
        let mut arr = vec![0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0]);
    }

    #[test]
    fn test_4() {
        // test code here
        let mut arr = vec![1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_5() {
        // test code here
        let mut arr = vec![1, 1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![1, 1]);
    }

    #[test]
    fn test_6() {
        // test code here
        let mut arr = vec![1, 0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 1]);
    }

    #[test]
    fn test_7() {
        // test code here
        let mut arr = vec![2];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![2]);
    }
}