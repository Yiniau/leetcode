#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        walker(s.as_bytes(), 1)
    }
}

fn walker(s: &[u8], count: i32) -> bool {
    let len = s.len();
    if len < 2 {
        return true;
    }

    if s[0] != s[len - 1] {
        if count == 0 {
            return false;
        }
        walker(&s[0..len - 1], count - 1) || walker(&s[1..len], count - 1)
    } else {
        walker(&s[1..len - 1], count)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ret = Solution::valid_palindrome(String::from("aba"));
        assert_eq!(ret, true)
    }
    #[test]
    fn test_2() {
        let ret = Solution::valid_palindrome(String::from("abca"));
        assert_eq!(ret, true)
    }
    #[test]
    fn test_3() {
        let ret = Solution::valid_palindrome(String::from("abc"));
        assert_eq!(ret, false)
    }
    #[test]
    fn test_4() {
        let ret = Solution::valid_palindrome(String::from("deeee"));
        assert_eq!(ret, true)
    }
    #[test]
    fn test_5() {
        let ret = Solution::valid_palindrome(String::from("cbbcc"));
        assert_eq!(ret, true)
    }
    #[test]
    fn test_6() {
        let ret = Solution::valid_palindrome(String::from("ebcbbbbcbe"));
        assert_eq!(ret, true)
    }
    #[test]
    fn test_7() {
        let ret = Solution::valid_palindrome(String::from("ececabbacec"));
        assert_eq!(ret, true)
    }
}
