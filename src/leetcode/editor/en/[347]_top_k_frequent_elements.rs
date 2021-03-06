//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given an integer array nums and an integer k, return the k most frequent 
//elements. You may return the answer in any order. 
//
// 
// Example 1: 
// Input: nums = [1,1,1,2,2,3], k = 2
//Output: [1,2]
// Example 2: 
// Input: nums = [1], k = 1
//Output: [1]
// 
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁵ 
// k is in the range [1, the number of unique elements in the array]. 
// It is guaranteed that the answer is unique. 
// 
//
// 
// Follow up: Your algorithm's time complexity must be better than O(n log n), 
//where n is the array's size. 
// Related Topics Array Hash Table Divide and Conquer Sorting Heap (Priority 
//Queue) Bucket Sort Counting Quickselect 👍 6570 👎 308


//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

#[derive(Eq)]
struct Node(i32, i32);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let Node(_, t1) = self;
        let Node(_, t2) = other;
        t1 == t2
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let Node(_, t1) = self;
        let Node(_, t2) = other;
        t1.cmp(t2)
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            let t = map.entry(num).or_insert(0);
            *t += 1;
        }

        let mut heap = BinaryHeap::from_iter(map.into_iter().map(|t| Node(t.0, t.1)));

        let mut group = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some(Node(key, _)) = heap.pop() {
                group.push(key);
            }
        }
        group
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(ret, vec![1, 2]);
    }

    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(ret, vec![1]);
    }
}