use std::cmp::min;

//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given a sorted array consisting of only integers where every element 
//appears exactly twice, except for one element which appears exactly once. Find 
//this single element that appears only once. 
//
// Follow up: Your solution should run in O(log n) time and O(1) space. 
//
// 
// Example 1: 
// Input: nums = [1,1,2,3,3,4,4,8,8]
//Output: 2
// Example 2: 
// Input: nums = [3,3,7,7,10,11,11]
//Output: 10
// 
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10^5 
// 0 <= nums[i] <= 10^5 
// 
// Related Topics Array Binary Search 👍 3158 👎 90


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 1 {
            unsafe { return *nums.get_unchecked(0); }
        }

        let mut i = 0;
        while i < length {
            if (i + 1 > length - 1) || (nums[i] != nums[i + 1]) {
                return nums[i];
            }
            i += 2;
        }
        nums[i - 1]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]);
        assert_eq!(ret, 10);
    }

    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::single_non_duplicate(vec![1, 1, 2]);
        assert_eq!(ret, 2);
    }
}