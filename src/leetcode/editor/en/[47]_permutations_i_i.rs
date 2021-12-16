//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given a collection of numbers, nums, that might contain duplicates, return 
//all possible unique permutations in any order. 
//
// 
// Example 1: 
//
// 
//Input: nums = [1,1,2]
//Output:
//[[1,1,2],
// [1,2,1],
// [2,1,1]]
// 
//
// Example 2: 
//
// 
//Input: nums = [1,2,3]
//Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 8 
// -10 <= nums[i] <= 10 
// 
// Related Topics Array Backtracking 👍 4071 👎 87


//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::iter::FromIterator;
use std::mem::swap;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut nums = nums;

        let mut stack = Vec::new();
        stack.resize(nums.len(), -100);
        // let mut stack = Vec::with_capacity(nums.len());
        let mut visited = Vec::new();
        visited.resize(nums.len(), false);

        nums.sort_unstable();
        Self::backtrack(&mut nums, 0, &mut ret, &mut stack, &mut visited);
        ret
    }

    fn backtrack(
        nums: &mut Vec<i32>,
        level: usize,
        ret: &mut Vec<Vec<i32>>,
        stack: &mut Vec<i32>,
        visited: &mut Vec<bool>,
    ) {
        let len = nums.len();
        if level == len {
            ret.push(stack.clone());
            return;
        }

        for i in 0..len {
            if visited[i] || (i > 0 && nums[i - 1] == nums[i] && !visited[i - 1]) {
                continue;
            }
            visited[i] = true;
            stack[level] = nums[i];
            // stack.push(nums[i]);
            Self::backtrack(nums, level + 1, ret, stack, visited);
            visited[i] = false;
            stack[level] = -100;
            // stack.pop();
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
        let ret = Solution::permute_unique(vec![1, 1, 2]);
        assert_eq!(ret, vec![vec![1, 1, 2],
                             vec![1, 2, 1],
                             vec![2, 1, 1]]);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::permute_unique(vec![2,2,1,1]);
        assert_eq!(ret, vec![vec![1,1,2,2],
                             vec![1,2,1,2],
                             vec![1,2,2,1],
                             vec![2,1,1,2],
                             vec![2,1,2,1],
                             vec![2,2,1,1]]);
    }
}