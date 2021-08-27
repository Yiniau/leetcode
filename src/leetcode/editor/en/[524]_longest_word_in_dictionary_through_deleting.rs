//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given a string s and a string array dictionary, return the longest string in 
//the dictionary that can be formed by deleting some of the given string 
//characters. If there is more than one possible result, return the longest word with the 
//smallest lexicographical order. If there is no possible result, return the empty 
//string. 
//
// 
// Example 1: 
//
// 
//Input: s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
//Output: "apple"
// 
//
// Example 2: 
//
// 
//Input: s = "abpcplea", dictionary = ["a","b","c"]
//Output: "a"
// 
//
// 
// Constraints: 
//
// 
// 1 <= s.length <= 1000 
// 1 <= dictionary.length <= 1000 
// 1 <= dictionary[i].length <= 1000 
// s and dictionary[i] consist of lowercase English letters. 
// 
// Related Topics Array Two Pointers String Sorting 👍 1137 👎 311


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dictionary = dictionary;
        dictionary.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(&b)));

        let chars = s.as_bytes();

        let mut ret = String::new();

        for str in dictionary.iter() {
            if str.len() < ret.len() { continue; }

            let b_str = str.as_bytes();
            let mut p = 0;

            for d_byte in chars {
                if p >= str.len() {
                    break;
                }
                if &b_str[p] == d_byte {
                    // match needed char, add p2
                    p += 1;
                }
            }

            if p == str.len() && str.len() > ret.len() {
                ret = str.clone();
            }
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::find_longest_word(String::from("abpcplea"), vec![String::from("ale"), String::from("apple"), String::from("monkey"), String::from("plea")]);
        assert_eq!(ret, String::from("apple"));
    }
    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::find_longest_word(String::from("abpcplea"), vec![String::from("a"), String::from("b"), String::from("monkey"), String::from("c")]);
        assert_eq!(ret, String::from("a"));
    }
    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::find_longest_word(String::from("abpcplea"), vec![String::from("")]);
        assert_eq!(ret, String::from(""));
    }
    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::find_longest_word(String::from("abce"), vec![String::from("abe"),String::from("abc")]);
        assert_eq!(ret, String::from("abc"));
    }
}