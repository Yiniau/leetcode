use std::collections::HashMap;

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
    pub fn main(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut l = 0;
        let mut new_l = 0;
        let mut len = 0;
        let mut map = HashMap::new();
        for (r, byte) in s.iter().enumerate() {
            map.entry(byte).and_modify(|t| {
                *t += 1
            }).or_insert(1);

            while map.len() > k as usize {
                map.entry(&s[new_l]).and_modify(|t| {
                    *t -= 1;
                });
                if let Some(t) = map.get(&s[new_l]) {
                    if t <= &0 {
                        map.remove(&s[new_l]);
                    }
                }
                new_l += 1;
            }

            if new_l < s.len() && len < r - new_l {
                len = r - new_l;
                l = new_l;
            }
        }
        String::from_utf8(s[l..(l + len + 1)].to_vec()).unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::main(String::from("eceba"), 2),
            "ece",
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::main(String::from("eceba"), 3),
            "eceb",
        );
    }
    #[test]
    fn test_2_1() {
        assert_eq!(
            Solution::main(String::from("eceeba"), 3),
            "eceeb",
        );
    }
    #[test]
    fn test_2_2() {
        assert_eq!(
            Solution::main(String::from("dceebeeeeeeea"), 3),
            "ceebeeeeeee",
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            Solution::main(String::from("ccaabbb"), 1),
            "bbb",
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            Solution::main(String::from("c"), 1),
            "c",
        );
    }
}
