//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an array nums of distinct integers, return all the possible 
//permutations. You can return the answer in any order. 
//
// 
// Example 1: 
// Input: nums = [1,2,3]
//Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// Example 2: 
// Input: nums = [0,1]
//Output: [[0,1],[1,0]]
// Example 3: 
// Input: nums = [1]
//Output: [[1]]
// 
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 6 
// -10 <= nums[i] <= 10 
// All the integers of nums are unique. 
// 
// Related Topics Array Backtracking 👍 8262 👎 160


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut nums = nums;
        let mut answer = Vec::with_capacity((len + 1) * len / 2);
        Self::backtrack(&mut nums, 0, &mut answer);
        answer
    }

    fn backtrack(nums: &mut Vec<i32>, level: usize, answer: &mut Vec<Vec<i32>>) {
        let len = nums.len();
        if level == len - 1 {
            answer.push(nums.clone());
            return;
        }

        for i in level..len {
            nums.swap(level, i);
            Self::backtrack(nums, level + 1, answer);
            nums.swap(level, i);
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
        let ret = Solution::permute(vec![1,2,3]);
        assert_eq!(ret, vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,1,2],
            vec![3,2,1],
        ]);
    }
}