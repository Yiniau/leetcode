//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an integer array nums and an integer k, return the kᵗʰ largest element 
//in the array. 
//
// Note that it is the kᵗʰ largest element in the sorted order, not the kᵗʰ 
//distinct element. 
//
// 
// Example 1: 
// Input: nums = [3,2,1,5,6,4], k = 2
//Output: 5
// Example 2: 
// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
//Output: 4
// 
// 
// Constraints: 
//
// 
// 1 <= k <= nums.length <= 10⁴ 
// -10⁴ <= nums[i] <= 10⁴ 
// 
// Related Topics Array Divide and Conquer Sorting Heap (Priority Queue) 
// Quickselect 👍 7382 👎 420


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 1. iterate nums, find the kᵗʰ position's number
        let mut l: usize = 0;
        let mut r = nums.len();
        let target_index = r - k as usize; // the index start from left
        let mut nums = nums;
        let mut pivot = l;
        while l < r {
            let new_pivot = sort(&mut nums, l, r); // range: [l..r)
            if new_pivot == target_index {
                pivot = new_pivot;
                break;
            } else if target_index > new_pivot {
                l = new_pivot + 1;
            } else {
                r = new_pivot; // because right of range is open, cannot reduce 1
            }
        };
        println!("pivot: {}", pivot);
        println!("{:?}", nums);
        nums[pivot]
    }
}

fn sort(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let mut i = l + 1;
    let mut j = r - 1;
    // make l as the quick sort pivot
    while l < r {
        while i < r && nums[l] > nums[i] {
            i += 1;
        }
        while l < j && nums[l] < nums[j] {
            j -= 1;
        }
        if i >= j {
            break;
        }
        nums.swap(i, j);
    }
    nums.swap(l, j);
    // how this swap work?
    // after the while loop, we can make sure i >= j, and nums[i] >= nums[j]
    // the pivot should be among by i and j,
    // because the both two sides of pivot are not been arranged, so just swap with j could work
    return j;
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);
        assert_eq!(ret, 5);
    }

    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
        assert_eq!(ret, 4);
    }
}