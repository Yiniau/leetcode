#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 */

//leetcode submit region begin(Prohibit modification and deletion)
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let origin_str_bytes = s.as_bytes();
        let target_str_bytes = t.as_bytes();

        let mut target_str_map =
            t.as_bytes()
                .iter()
                .fold(HashMap::new(), |mut a: HashMap<u8, i32>, &b| {
                    a.entry(b).and_modify(|t| *t += 1).or_insert(1);
                    a
                });

        let need_count = target_str_bytes.len();
        let mut count: usize = 0;
        let mut l: usize = 0;
        let mut min_l: usize = 0;

        let mut size: usize = origin_str_bytes.len();

        for (r, c) in origin_str_bytes.iter().enumerate() {
            // 1. make window contain all char in target string
            target_str_map.entry(*c).and_modify(|t| {
                *t -= 1; // cound reduce all the time, could limit max touched time while move l
                if *t >= 0 {
                    count += 1;
                }
            });

            // 2. *try* move left point to right to get minimum window size
            while count >= need_count {
                // only apply changes when the situation meets the requirements of the problem
                if l > min_l {
                    min_l = l;
                    size = r - l + 1;
                }

                target_str_map.entry(origin_str_bytes[l]).and_modify(|t| {
                    *t += 1;
                    if *t > 0 {
                        count -= 1;
                    }
                });
                l += 1; // must after minus target char touch count logic
            }
        }

        String::from_utf8(origin_str_bytes[min_l..min_l + size].to_vec()).unwrap()
    }
}
// @lc code=end
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use crate::minimum_window_substring::Solution;

    #[test]
    fn test_1() {
        let ret = Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC"));
        assert_eq!(ret, String::from("BANC"));
    }
}
