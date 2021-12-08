//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//There are n cities. Some of them are connected, while some are not. If city a 
//is connected directly with city b, and city b is connected directly with city c,
// then city a is connected indirectly with city c. 
//
// A province is a group of directly or indirectly connected cities and no 
//other cities outside of the group. 
//
// You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the 
//iᵗʰ city and the jᵗʰ city are directly connected, and isConnected[i][j] = 0 
//otherwise. 
//
// Return the total number of provinces. 
//
// 
// Example 1: 
//
// 
//Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
//Output: 2
// 
//
// Example 2: 
//
// 
//Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
//Output: 3
// 
//
// 
// Constraints: 
//
// 
// 1 <= n <= 200 
// n == isConnected.length 
// n == isConnected[i].length 
// isConnected[i][j] is 1 or 0. 
// isConnected[i][i] == 1 
// isConnected[i][j] == isConnected[j][i] 
// 
// Related Topics Depth-First Search Breadth-First Search Union Find Graph 👍 42
//21 👎 214


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut is_connected = is_connected;

        let mut circle_count = 0;

        let len = is_connected.len();
        for i in 0..len {
            if is_connected[i][i] == 1 {
                circle_count += 1;
                Self::search(&mut is_connected, i);
            }
        }

        circle_count
    }
    fn search(grid: &mut Vec<Vec<i32>>, i: usize) {
        if grid[i][i] == 0 {  return; }
        grid[i][i] = 0;
        for t in 0..grid.len() {
            if grid[i][t] == 1 {
                Self::search(grid, t);
            }
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
        let ret = Solution::find_circle_num(vec![
            vec![1, 1, 0],
            vec![1, 1, 0],
            vec![0, 0, 1],
        ]);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_2() {
        // test code here
        let ret = Solution::find_circle_num(vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ]);
        assert_eq!(ret, 3);
    }

    #[test]
    fn test_3() {
        // test code here
        let ret = Solution::find_circle_num(vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
        ]);
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_4() {
        // test code here
        let ret = Solution::find_circle_num(vec![
            vec![1, 0, 1, 1],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![1, 0, 0, 1],
        ]);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_5() {
        // test code here
        let ret = Solution::find_circle_num(vec![
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
        ]);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_6() {
        // test code here
        let ret = Solution::find_circle_num(vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ]);
        assert_eq!(ret, 8);
    }
}