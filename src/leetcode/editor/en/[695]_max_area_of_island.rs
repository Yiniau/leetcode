//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given an m x n binary matrix grid. An island is a group of 1's (
//representing land) connected 4-directionally (horizontal or vertical.) You may assume 
//all four edges of the grid are surrounded by water. 
//
// The area of an island is the number of cells with a value 1 in the island. 
//
// Return the maximum area of an island in grid. If there is no island, return 0
//. 
//
// 
// Example 1: 
//
// 
//Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,
//0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,
//0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]
//]
//Output: 6
//Explanation: The answer is not 11, because the island must be connected 4-
//directionally.
// 
//
// Example 2: 
//
// 
//Input: grid = [[0,0,0,0,0,0,0,0]]
//Output: 0
// 
//
// 
// Constraints: 
//
// 
// m == grid.length 
// n == grid[i].length 
// 1 <= m, n <= 50 
// grid[i][j] is either 0 or 1. 
// 
// Related Topics Array Depth-First Search Breadth-First Search Union Find 
//Matrix 👍 4788 👎 123


//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let direction: [i32; 5] = [-1, 0, 1, 0, -1];

        let row_length = grid.len();
        let column_length = grid[0].len();

        let mut area = 0;

        for r in 0..row_length {
            for c in 0..column_length {
                if grid[r][c] == 1 {
                    let mut stack = vec![];
                    stack.push((r, c));
                    let mut _area = 1;
                    while !stack.is_empty() {
                        let (r, c) = stack.pop().unwrap();
                        grid[r][c] = 0;
                        for i in 0..4 {
                            let y = r as i32 + direction[i];
                            let x = c as i32 + direction[i + 1];
                            if x < 0 || y < 0 {
                                continue;
                            }
                            let uy = y as usize;
                            let ux = x as usize;
                            if ux < column_length && uy < row_length && grid[uy][ux] == 1 {
                                _area += 1;
                                stack.push((uy, ux));
                                grid[uy][ux] = 0;
                            }
                        }
                    }
                    area = max(_area, area);
                    _area = 0;
                }
            }
        }
        area
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // test code here
        let ret = Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]);
        assert_eq!(ret, 6);
    }
}