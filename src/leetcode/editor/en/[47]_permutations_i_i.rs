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
        let mut map = HashMap::new();
        let mut nums = nums;
        Self::backtrack(&mut nums, 0, &mut map);
        map.values().cloned().collect()
        // map.into_iter().
    }

    fn backtrack(nums: &mut Vec<i32>, level: usize, ret: &mut HashMap<String, Vec<i32>>) {
        let current_num_key = Self::toString(nums);
        if level == nums.len() - 1 {
            ret.entry(current_num_key).or_insert(nums.clone());
        }
        for i in level..nums.len() {
            nums.swap(level, i);
            Self::backtrack(nums, level + 1, ret);
            nums.swap(level, i);
        }
    }

    fn toString(nums: &mut Vec<i32>) -> String {
        let mut t = Vec::new();
        for num in nums {
            t.append(&mut num.to_be_bytes().to_vec());
        }
        unsafe { String::from_utf8_unchecked(t) }
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
}