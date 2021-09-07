//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//There is an integer array nums sorted in non-decreasing order (not 
//necessarily with distinct values). 
//
// Before being passed to your function, nums is rotated at an unknown pivot 
//index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1]
//, ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0
//,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,
//2,4,4]. 
//
// Given the array nums after the rotation and an integer target, return true 
//if target is in nums, or false if it is not in nums. 
//
// You must decrease the overall operation steps as much as possible. 
//
// 
// Example 1: 
// Input: nums = [2,5,6,0,0,1,2], target = 0
//Output: true
// Example 2: 
// Input: nums = [2,5,6,0,0,1,2], target = 3
//Output: false
// 
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 5000 
// -10⁴ <= nums[i] <= 10⁴ 
// nums is guaranteed to be rotated at some pivot. 
// -10⁴ <= target <= 10⁴ 
// 
//
// 
// Follow up: This problem is similar to Search in Rotated Sorted Array, but 
//nums may contain duplicates. Would this affect the runtime complexity? How and why?
// 
// Related Topics Array Binary Search 👍 2627 👎 614


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // nums.contains(&target)

        let mut start = 0;
        let mut end = nums.len();

        while end > start {
            let mid = (start + (end - 1)) / 2;
            let mid_num = unsafe { nums.get_unchecked(mid) };
            if mid_num == &target { return true };
            let start_num = unsafe { nums.get_unchecked(start) };
            if start_num == &target { return true };
            let end_num = unsafe { nums.get_unchecked(end - 1) };
            if end_num == &target { return true };

            // the left side of array is sorted by ascending
            if mid_num > start_num {
                if binary_search::<i32>(nums[start..mid].to_vec(), &target) {
                    return true;
                } else {
                    start = mid + 1;
                    continue;
                }
            }
            if mid_num < end_num {
                if binary_search::<i32>(nums[mid..end].to_vec(), &target) {
                    return true;
                } else {
                    end = mid;
                    continue;
                }
            }
            if mid_num == start_num {
                start += 1;
            }
            if mid_num == end_num {
                end -= 1;
            }
        }

        false
    }
}

fn binary_search<T>(nums: Vec<T>, target: &T) -> bool
where T: std::cmp::PartialEq + std::cmp::PartialOrd
{
    // nums.binary_search()
    if nums.len() == 0 {
        return false;
    }
    let mut l = 0;
    let mut r = nums.len();
    if r == 1 {
        unsafe { return nums.get_unchecked(l) == target; }
    }
    while r > l {
        let m = l + (r - l) / 2;
        let num_m = unsafe { nums.get_unchecked(m) };
        if num_m == target { return true; }
        if num_m > target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    false
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::search(vec![2,5,6,0,0,1,2], 0);
        assert_eq!(ret, true);
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::search(vec![2,5,6,0,0,1,2], 3);
        assert_eq!(ret, false);
    }
    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::search(vec![1], 1);
        assert_eq!(ret, true);
    }
    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::search(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1], 2);
        assert_eq!(ret, true);
    }
    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::search(vec![1,1,1,1,1,1,1,1,1,13,1,1,1,1,1,1,1,1,1,1,1,1], 13);
        assert_eq!(ret, true);
    }
    #[test]
    fn test_6() {
        // test code here
        let ret = Solution::search(vec![3,1,1], 0);
        assert_eq!(ret, false);
    }
}