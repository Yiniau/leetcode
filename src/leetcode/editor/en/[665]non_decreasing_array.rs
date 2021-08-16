//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an array nums with n integers, your task is to check if it could become 
//non-decreasing by modifying at most one element. 
//
// We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for ever
//y i (0-based) such that (0 <= i <= n - 2). 
//
// 
// Example 1: 
//
// 
//Input: nums = [4,2,3]
//Output: true
//Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
// 
//
// Example 2: 
//
// 
//Input: nums = [4,2,1]
//Output: false
//Explanation: You can't get a non-decreasing array by modify at most one elemen
//t.
// 
//
// 
// Constraints: 
//
// 
// n == nums.length 
// 1 <= n <= 104 
// -105 <= nums[i] <= 105 
// 
// Related Topics Array 
// 👍 3219 👎 646



//leetcode submit region begin(Prohibit modification and deletion)
// the key of solution is how to decide change current value or another value
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 1;
        let mut nums = nums;
        let length = nums.len();

        if length <= 2 {
            return true;
        }

        if nums[0] > nums[1] {
            nums[0] = nums[1];
            count -= 1;
        }

        // for (i, num) in nums[0..length - 2].iter_mut().enumerate() {
        for i in 2..length {
            if count < 0 {
                break;
            }
            if nums[i - 1] > nums[i] {
                count -= 1;
                if nums[i - 2] > nums[i] {
                    nums[i] = nums[i - 1];
                } else {
                    nums[i - 1] = nums[i];
                }
            }
        }

        count >= 0
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use crate::non_decreasing_array::Solution;

    #[test]
    fn test_1() {
        // test code here
        assert_eq!(Solution::check_possibility(vec![4,2,3]), true);
    }
    #[test]
    fn test_2() {
        // test code here
        assert_eq!(Solution::check_possibility(vec![4,2,1]), false);
    }
    #[test]
    fn test_3() {
        // test code here
        assert_eq!(Solution::check_possibility(vec![3,4,2,3]), false);
    }
    #[test]
    fn test_4() {
        // test code here
        assert_eq!(Solution::check_possibility(vec![2,3,3,2,2]), false);
    }
    #[test]
    fn test_5() {
        // test code here
        assert_eq!(Solution::check_possibility(vec![5,7,1,8]), true);
    }
}