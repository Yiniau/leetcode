//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Suppose an array of length n sorted in ascending order is rotated between 1 
//and n times. For example, the array nums = [0,1,4,4,5,6,7] might become: 
//
// 
// [4,5,6,7,0,1,4] if it was rotated 4 times. 
// [0,1,4,4,5,6,7] if it was rotated 7 times.
// 
//
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results 
//in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]]. 
//
// Given the sorted rotated array nums that may contain duplicates, return the 
//minimum element of this array. 
//
// You must decrease the overall operation steps as much as possible. 
//
// 
// Example 1: 
// Input: nums = [1,3,5]
//Output: 1
// Example 2: 
// Input: nums = [2,2,2,0,1]
//Output: 0
// 
// 
// Constraints: 
//
// 
// n == nums.length 
// 1 <= n <= 5000 
// -5000 <= nums[i] <= 5000 
// nums is sorted and rotated between 1 and n times. 
// 
//
// 
// Follow up: This problem is similar to Find Minimum in Rotated Sorted Array, 
//but nums may contain duplicates. Would this affect the runtime complexity? How 
//and why? 
//
// 
// Related Topics Array Binary Search 👍 1892 👎 295


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut ret = i32::MAX;

        let mut l = 0;
        let mut r = nums.len();

        while r > l {
            let p = (l + ( r - 1)) / 2;
            let l_v = nums[l];
            let r_v = nums[r - 1];
            let m_v = nums[p];
            let mut v_group = [ret, l_v, r_v, m_v];
            v_group.sort_by(|a, b| a.cmp(b));
            ret = v_group[0];

            if m_v < r_v { // at p, the right side of nums was sorted by ascending
                r = p;
                continue;
            }

            if m_v > l_v { // at p, the left side of nums was sorted by ascending
                l = p + 1;
                continue;
            }

            if m_v == l_v {
                l += 1;
            }
            if m_v == r_v {
                r -= 1;
            }
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::find_min(vec![1,3,5]);
        assert_eq!(ret, 1);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::find_min(vec![2,2,2,0,1]);
        assert_eq!(ret, 0);
    }
    #[test]
    fn test_3() {
        let ret = Solution::find_min(vec![1,1,1,1,1,2,1,1]);
        assert_eq!(ret, 1);
    }
    #[test]
    fn test_4() {
        let ret = Solution::find_min(vec![3,1,1]);
        assert_eq!(ret, 1);
    }
    #[test]
    fn test_5() {
        let ret = Solution::find_min(vec![1,3,3]);
        assert_eq!(ret, 1);
    }
}