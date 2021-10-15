//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given two sorted arrays nums1 and nums2 of size m and n respectively, return 
//the median of the two sorted arrays. 
//
// The overall run time complexity should be O(log (m+n)). 
//
// 
// Example 1: 
//
// 
//Input: nums1 = [1,3], nums2 = [2]
//Output: 2.00000
//Explanation: merged array = [1,2,3] and median is 2.
// 
//
// Example 2: 
//
// 
//Input: nums1 = [1,2], nums2 = [3,4]
//Output: 2.50000
//Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
// 
//
// Example 3: 
//
// 
//Input: nums1 = [0,0], nums2 = [0,0]
//Output: 0.00000
// 
//
// Example 4: 
//
// 
//Input: nums1 = [], nums2 = [1]
//Output: 1.00000
// 
//
// Example 5: 
//
// 
//Input: nums1 = [2], nums2 = []
//Output: 2.00000
// 
//
// 
// Constraints: 
//
// 
// nums1.length == m 
// nums2.length == n 
// 0 <= m <= 1000 
// 0 <= n <= 1000 
// 1 <= m + n <= 2000 
// -10⁶ <= nums1[i], nums2[i] <= 10⁶ 
// 
// Related Topics Array Binary Search Divide and Conquer 👍 12782 👎 1723



//leetcode submit region begin(Prohibit modification and deletion)
fn get_median(nums: &Vec<i32>) -> f64 {
    let len = nums.len();
    let try_median = (len - 1) / 2;
    return if (len - 1) as f64 / 2_f64 > try_median as f64 {
        (nums[try_median] + nums[try_median + 1] / 2) as f64
    } else {
        nums[try_median] as f64
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut ret = 0_f64;

        let l_1 = nums1.len();
        let l_2 = nums2.len();

        let mut p_1_l = 0;
        let mut p_1_r = l_1;
        let mut p_2_l = 0;
        let mut p_2_r = l_2;

        if l_1 == 0 {
            return get_median(&nums2);
        }
        if l_2 == 0 {
            return get_median(&nums1);
        }

        while p_1_l <= p_1_r && p_2_l <= p_2_r {

        }

        return ret;
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let ret = Solution::find_median_sorted_arrays(vec![1,3], vec![2]);
        assert_eq!(ret, 2.0000);
    }
    #[test]
    fn test_1() {
        let ret = Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]);
        assert_eq!(ret, 2.5000);
    }
    #[test]
    fn test_2() {
        let ret = Solution::find_median_sorted_arrays(vec![0,0], vec![0,0]);
        assert_eq!(ret, 0.0000);
    }
    #[test]
    fn test_3() {
        let ret = Solution::find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(ret, 1.0000);
    }
    #[test]
    fn test_4() {
        let ret = Solution::find_median_sorted_arrays(vec![2], vec![1]);
        assert_eq!(ret, 2.0000);
    }
}