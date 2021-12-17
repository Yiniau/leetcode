//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//Given two integers n and k, return all possible combinations of k numbers out 
//of the range [1, n]. 
//
// You may return the answer in any order. 
//
// 
// Example 1: 
//
// 
//Input: n = 4, k = 2
//Output:
//[
//  [2,4],
//  [3,4],
//  [2,3],
//  [1,2],
//  [1,3],
//  [1,4],
//]
// 
//
// Example 2: 
//
// 
//Input: n = 1, k = 1
//Output: [[1]]
// 
//
// 
// Constraints: 
//
// 
// 1 <= n <= 20 
// 1 <= k <= n 
// 
// Related Topics Array Backtracking 👍 3319 👎 111


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let range: Vec<i32> = (1..(n + 1)).collect();

        let mut ret = Vec::new();
        let mut stack = Vec::new();
        stack.resize(k as usize, -1);

        Self::backtrack(&range, 0, &mut ret, &mut stack, n, k, 0);

        ret
    }

    fn backtrack(
        range: &Vec<i32>,
        level: usize,
        ret: &mut Vec<Vec<i32>>,
        stack: &mut Vec<i32>,
        n: i32,
        k: i32,
        previou_i: usize,
    ) {
        if level == stack.len() {
            ret.push(stack.clone());
            return;
        }
        for i in level..(n as usize - k as usize + 1 + level) {
            let num = range[i];
            if level > 0 && previou_i >= i {
                continue;
            }
            stack[level] = num;
            Self::backtrack(range, level + 1, ret, stack, n, k, i);
            stack[level] = -1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::combine(4, 2);
        assert_eq!(ret, vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ]);
    }

    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::combine(1, 1);
        assert_eq!(ret, vec![vec![1]]);
    }

    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::combine(3, 2);
        assert_eq!(ret, vec![
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ]);
    }

    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::combine(2, 1);
        assert_eq!(ret, vec![
            vec![1],
            vec![2],
        ]);
    }

    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::combine(3, 3);
        assert_eq!(ret, vec![
            vec![1, 2, 3],
        ]);
    }

    #[test]
    fn test_6() {
        // test code here
        let ret = Solution::combine(4, 3);
        assert_eq!(ret, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
        ]);
    }

    #[test]
    fn test_7() {
        // test code here
        let ret = Solution::combine(5, 3);
        assert_eq!(ret, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ]);
    }
}