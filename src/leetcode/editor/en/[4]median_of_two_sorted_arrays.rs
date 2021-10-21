use std::cmp::max;

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
impl Solution {
    /// nums1 and nums2 can be divided into two part of equal length
    /// left_part          |        right_part
    /// A[0], A[1], ..., A[i-1]  |  A[i], A[i+1], ..., A[m-1]
    /// B[0], B[1], ..., B[j-1]  |  B[j], B[j+1], ..., B[n-1]
    /// if we can ensure
    /// 1. A[i-1] <= B[j]
    /// 2. B[j-1] <= A[i]
    /// this mean we have found the median of nums1 and nums2
    /// so this function's target is found the i and j make
    /// 1. i + j = m - i + n - j (or m - i + n - j + 1)
    /// 2. A[i-1] <= B[j] && B[j-1] <= A[i]
    /// median =
    ///     max(A[i-1], B[j-1]) (when m + n is odd)
    ///     or (max(A[i-1], B[j-1]) + min(A[i], B[j]))/2 (when m + n is even)
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut ret = 0_f64;

        // 1. make i and j
        // set
        //  m = len(first array)
        //  n = len(second array)
        // because
        //  we get j from i and
        //  len(left_part) == len(right_part)
        // therefore
        //  if m + n is odd
        //      i + j = m - i + n - j + 1
        //  if m + n is even
        //      i + j = m - i + n - j
        // therefore
        //  if m + n is odd
        //      2j = m + n - 2i + 1
        //  if m + n is even
        //      2j = m + n - 2i
        // because int / int = int
        // therefore j = (m + n + 1)/ 2 - i
        // but n >= j >= 0
        // (m + n + 1)/ 2 - i >= 0, m >= i
        // => (m + n + 1) / 2 - m >= (m + n + 1)/ 2 - i >= 0
        // => (n - m + 1) / 2 >= 0
        // => n - m >= 0
        // => n >= m

        // 1.1 make arrays
        let (a, b): (Vec<i32>, Vec<i32>) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        // 1.2 make m,n
        let m = a.len();
        let n = b.len();

        // 1.3 make loop
        let mut imin = 0;
        let mut imax = m;
        while imin <= imax {
            let mut i = (imin + imax) / 2;
            let mut j = (m + n + 1) / 2 - i;

            // 2 Now we have len(left_part)==len(right_part). And there are only 3 situations
            //      that we may encounter:
            //     <a> b[j-1] <= a[i] and a[i-1] <= b[j]
            //         Means we have found the object `i`, so stop searching.
            //     <b> b[j-1] > a[i]
            //         Means a[i] is too small. We must `ajust` i to get `b[j-1] <= a[i]`.
            //         Can we `increase` i?
            //             Yes. Because when i is increased, j will be decreased.
            //             So b[j-1] is decreased and a[i] is increased, and `b[j-1] <= a[i]` may
            //             be satisfied.
            //         Can we `decrease` i?
            //             `No!` Because when i is decreased, j will be increased.
            //             So b[j-1] is increased and a[i] is decreased, and b[j-1] <= a[i] will
            //             be never satisfied.
            //         So we must `increase` i. That is, we must ajust the searching range to
            //         [i+1, imax]. So, set imin = i+1, and goto <2>.
            //     <c> a[i-1] > b[j]
            //         Means a[i-1] is too big. And we must `decrease` i to get `a[i-1]<=b[j]`.
            //         That is, we must ajust the searching range to [imin, i-1].
            //         So, set imax = i-1, and goto <2>.
            //
            // Now let's consider the edges values i=0,i=m,j=0,j=n where a[i-1],b[j-1],a[i],b[j]
            // What we need to do is ensuring that max(left_part) <= min(right_part).
            // So, if i and j are not edges values(means a[i-1],b[j-1],a[i],b[j] all exist),
            // then we must check both b[j-1] <= a[i] and a[i-1] <= b[j].
            // But if some of a[i-1],b[j-1],a[i],b[j] don't exist,
            // then we don't need to check one(or both) of these two conditions.
            // For example, if i=0, then a[i-1] doesn't exist, then we don't need to check a[i-1] <= b[j].
            // So, what we need to do is:
            // Searching i in [0, m], to find an object `i` that:
            //     (j == 0 or i == m or b[j-1] <= a[i]) and
            //     (i == 0 or j == n or a[i-1] <= b[j])
            //     where j = (m + n + 1)/2 - i
            //
            // And in a searching loop, we will encounter only three situations:
            // <a> (j == 0 or i == m or b[j-1] <= a[i]) and
            //     (i == 0 or j = n or a[i-1] <= b[j])
            //     Means i is perfect, we can stop searching.
            //
            // <b> j > 0 and i < m and b[j - 1] > a[i]
            //     Means i is too small, we must increase it.
            //
            // <c> i > 0 and j < n and a[i - 1] > b[j]
            //     Means i is too big, we must decrease it.

            if (j == 0 || i == m || (b[j - 1] <= a[i])) && (i == 0 || j == n || (a[i - 1] <= b[j])) {
                // 2.<a>
                let max_left_part: i32;
                let min_right_part: i32;

                if i == 0 {
                    max_left_part = b[j - 1];
                } else if j == 0 {
                    max_left_part = a[i - 1];
                } else {
                    max_left_part = max(a[i - 1], b[j - 1]);
                }

                ret = if (m + n) % 2 == 1 {
                    max_left_part as f64
                } else {
                    if i == m {
                        min_right_part = b[j];
                    } else if j == n {
                        min_right_part = a[i];
                    } else {
                        min_right_part = a[i].min(b[j]);
                    }
                    ((max_left_part + min_right_part) as f64) / 2_f64
                };
                break;
            } else if j > 0 && i < m && b[j - 1] > a[i] {
                // 2.<b>
                // increase imin
                imin = i + 1;
            } else if i > 0 && j < n && a[i - 1] > b[j] {
                // 2.<c>
                // reduce imax
                imax = i - 1;
            }
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
        let ret = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(ret, 2.0000);
    }

    #[test]
    fn test_1() {
        let ret = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(ret, 2.5000);
    }

    #[test]
    fn test_2() {
        let ret = Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]);
        assert_eq!(ret, 0.0000);
    }

    #[test]
    fn test_3() {
        let ret = Solution::find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(ret, 1.0000);
    }

    #[test]
    fn test_4() {
        let ret = Solution::find_median_sorted_arrays(vec![2], vec![]);
        assert_eq!(ret, 2.0000);
    }
}