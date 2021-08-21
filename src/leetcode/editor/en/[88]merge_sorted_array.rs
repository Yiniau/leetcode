//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given two integer arrays nums1 and nums2, sorted in non-decreasing
//order, and two integers m and n, representing the number of elements in nums1 and
//nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead
//be stored inside the array nums1. To accommodate this, nums1 has a length of m +
//n, where the first m elements denote the elements that should be merged, and the
//last n elements are set to 0 and should be ignored. nums2 has a length of n.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//Output: [1,2,2,3,5,6]
//Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
//The result of the merge is [1,2,2,3,5,6] with the underlined elements coming
//from nums1.
//
//
// Example 2:
//
//
//Input: nums1 = [1], m = 1, nums2 = [], n = 0
//Output: [1]
//Explanation: The arrays we are merging are [1] and [].
//The result of the merge is [1].
//
//
// Example 3:
//
//
//Input: nums1 = [0], m = 0, nums2 = [1], n = 1
//Output: [1]
//Explanation: The arrays we are merging are [] and [1].
//The result of the merge is [1].
//Note that because m = 0, there are no elements in nums1. The 0 is only there
//to ensure the merge result can fit in nums1.
//
//
//
// Constraints:
//
//
// nums1.length == m + n
// nums2.length == n
// 0 <= m, n <= 200
// 1 <= m + n <= 200
// -10⁹ <= nums1[i], nums2[j] <= 10⁹
//
//
//
// Follow up: Can you come up with an algorithm that runs in O(m + n) time?
// Related Topics Array Two Pointers Sorting 👍 874 👎 112

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }
        let mut p1: usize = 0;
        let mut p2: usize = 0;
        let m = m as usize;
        let n = n as usize;
        while p1 < m + p2 {
            if p2 < n && &nums1[p1] >= &nums2[p2] {
                nums1.insert(p1, nums2[p2]);
                p2 += 1;
            }
            p1 += 1;
        }
        while p2 < n {
            nums1[p1] = nums2[p2];
            p2 += 1;
            p1 += 1;
        }
        nums1.drain(p1..);
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use crate::merge_sorted_array::Solution;

    #[test]
    fn test_1() {
        let mut n1 = vec![1, 2, 3, 0, 0, 0];
        let mut n2 = vec![2, 5, 6];
        Solution::merge(&mut n1, 3, &mut n2, 3);
        assert_eq!(n1, vec![1, 2, 2, 3, 5, 6],);
    }
    #[test]
    fn test_2() {
        let mut n1 = vec![0];
        let mut n2 = vec![1];
        Solution::merge(&mut n1, 0, &mut n2, 1);
        assert_eq!(n1, vec![1],);
    }
    #[test]
    fn test_3() {
        let mut n1 = vec![1];
        let mut n2 = vec![];
        Solution::merge(&mut n1, 1, &mut n2, 0);
        assert_eq!(n1, vec![1],);
    }
    #[test]
    fn test_4() {
        let mut n1 = vec![2,0];
        let mut n2 = vec![1];
        Solution::merge(&mut n1, 1, &mut n2, 1);
        assert_eq!(n1, vec![1,2],);
    }
}
