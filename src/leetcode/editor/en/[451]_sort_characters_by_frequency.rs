//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given a string s, sort it in decreasing order based on the frequency of the 
//characters. The frequency of a character is the number of times it appears in the 
//string. 
//
// Return the sorted string. If there are multiple answers, return any of them. 
//
//
// 
// Example 1: 
//
// 
//Input: s = "tree"
//Output: "eert"
//Explanation: 'e' appears twice while 'r' and 't' both appear once.
//So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid 
//answer.
// 
//
// Example 2: 
//
// 
//Input: s = "cccaaa"
//Output: "aaaccc"
//Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and 
//"aaaccc" are valid answers.
//Note that "cacaca" is incorrect, as the same characters must be together.
// 
//
// Example 3: 
//
// 
//Input: s = "Aabb"
//Output: "bbAa"
//Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
//Note that 'A' and 'a' are treated as two different characters.
// 
//
// 
// Constraints: 
//
// 
// 1 <= s.length <= 5 * 10⁵ 
// s consists of uppercase and lowercase English letters and digits. 
// 
// Related Topics Hash Table String Sorting Heap (Priority Queue) Bucket Sort 
//Counting 👍 3434 👎 171


//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

#[derive(Eq)]
struct Node(u8, i32);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
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
    pub fn frequency_sort(s: String) -> String {
        let mut ret = Vec::with_capacity(s.len());
        let mut map = HashMap::with_capacity(s.len());
        for &byte in s.as_bytes() {
            *(map.entry(byte).or_insert(0)) += 1;
        }
        // let mut sorted = Vec::with_capacity(s.len());
        let mut heap = BinaryHeap::from_iter((map.into_iter().map(|t| Node(t.0, t.1))));
        while let Some(Node(key, value)) = heap.pop() {
            for _ in 0..value {
                ret.push(key);
            }
        }
        String::from_utf8(ret).unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::frequency_sort(String::from("tree"));
        assert!(ret == String::from("eert") || ret == String::from("eetr"));
    }
11
    #[test]
    fn test_2() {
        let ret = Solution::frequency_sort(String::from("Aabb"));
        assert!(ret == String::from("bbAa") || ret == String::from("bbaA"));
    }

    #[test]
    fn test_3() {
        let ret = Solution::frequency_sort(String::from("cccaaa"));
        assert!(ret == String::from("aaaccc") || ret == String::from("cccaaa"));
    }
}