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
use std::cmp::Ordering;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let p_l = lower_bound(&nums, target);
        let p_r = upper_bound(&nums, target); // need to minus 1

        if p_l == nums.len() || nums[p_l] != target {
            return vec![-1,-1];
        }

        vec![p_l as i32, p_r as i32 - 1]
    }
}
// return the first target item in vector
// the key of function is `nums[p] >= target`
// while condition fit `=`, make right side point equal to middle point
// so at the end, the l while be first target item
// example:
//   input: [1,2,2]
//   left point: 0
//   right point: 2
// first while round, p = (0 + 2) / 2 = 1
// nums[p] == 2 == target
//   make r = 1
//        l = 0
// round 2, p = (0 + 1) / 2 = 0
//   make l = p + 1 = 0 + 1 = 1
// If we always move the right pointer to the left
// when we encounter nums[p] >= target,
// then eventually the right pointer will converge to the position
// where the last target appears in the iterator
fn lower_bound(nums: &Vec<i32>, target: i32) -> usize {
    let mut l = 0;
    let mut r = nums.len(); // don't set it as the last index, we can use it to check is there where a target item exist in the nums
    let mut p = 0;
    while l < r {
        p = (l + r - 1) / 2;
        if nums[p] >= target {
            r = p;
        } else {
            l = p + 1;
        }
    }
    l
}

fn upper_bound(nums: &Vec<i32>, target: i32) -> usize {
    let mut l = 0;
    let mut r = nums.len();
    let mut p = 0;
    while l < r {
        p = (l + r - 1) / 2;
        if nums[p] > target {
            r = p;
        } else {
            l = p + 1;
        }
    }
    l
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
    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::search_range(vec![2,2],2);
        assert_eq!(ret, vec![0,1]);
    }
    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::search_range(vec![1,4],4);
        assert_eq!(ret, vec![1,1]);
    }
}