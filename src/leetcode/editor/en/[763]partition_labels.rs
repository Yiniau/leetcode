//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given a string s. We want to partition the string into as many parts a
//s possible so that each letter appears in at most one part.
//
// Return a list of integers representing the size of these parts.
//
//
// Example 1:
//
//
//Input: s = "ababcbacadefegdehijhklij"
//Output: [9,7,8]
//Explanation:
//The partition is "ababcbaca", "defegde", "hijhklij".
//This is a partition so that each letter appears in at most one part.
//A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it split
//s s into less parts.
//
//
// Example 2:
//
//
//Input: s = "eccbbbbdec"
//Output: [10]
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 500
// s consists of lowercase English letters.
//
// Related Topics Hash Table Two Pointers String Greedy
// 👍 4949 👎 202

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut last = [0i32; 32];
        for (i, byte) in s.bytes().enumerate() {
            last[byte as usize - 97usize] = i as i32; // 用index表示字母，a == 97 - 97, b = 98 - 97
        }

        let mut index = 0i32;
        let mut start = 0i32;
        for (i, byte) in s.bytes().enumerate() {
            if index < last[byte as usize - 97usize] {
                index = last[byte as usize - 97usize];
            }
            if i as i32 == index {
                result.push(i as i32 - start + 1i32);
                start = i as i32 + 1i32;
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use crate::partition_labels::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::partition_labels(String::from("ababcbacadefegdehijhklij")),
            vec![9, 7, 8],
        );
    }
}
