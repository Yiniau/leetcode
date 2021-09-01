//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an array of integers nums sorted in ascending order, find the starting 
//and ending position of a given target value. 
//
// If target is not found in the array, return [-1, -1]. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
// Input: nums = [5,7,7,8,8,10], target = 8
//Output: [3,4]
// Example 2: 
// Input: nums = [5,7,7,8,8,10], target = 6
//Output: [-1,-1]
// Example 3: 
// Input: nums = [], target = 0
//Output: [-1,-1]
// 
// 
// Constraints: 
//
// 
// 0 <= nums.length <= 10⁵ 
// -10⁹ <= nums[i] <= 10⁹ 
// nums is a non-decreasing array. 
// -10⁹ <= target <= 10⁹ 
// 
// Related Topics Array Binary Search 👍 6994 👎 231


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::search_range(vec![5,7,7,8,8,10],8);
        assert_eq!(ret, vec![3,4]);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::search_range(vec![5,7,7,8,8,10],6);
        assert_eq!(ret, vec![-1,-1]);
    }
    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::search_range(vec![],0);
        assert_eq!(ret, vec![-1,-1]);
    }
}