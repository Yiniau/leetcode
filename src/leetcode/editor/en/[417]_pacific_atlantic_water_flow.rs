//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//There is an m x n rectangular island that borders both the Pacific Ocean and 
//Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and 
//the Atlantic Ocean touches the island's right and bottom edges. 
//
// The island is partitioned into a grid of square cells. You are given an m x 
//n integer matrix heights where heights[r][c] represents the height above sea 
//level of the cell at coordinate (r, c). 
//
// The island receives a lot of rain, and the rain water can flow to 
//neighboring cells directly north, south, east, and west if the neighboring cell's height 
//is less than or equal to the current cell's height. Water can flow from any cell 
//adjacent to an ocean into the ocean. 
//
// Return a 2D list of grid coordinates result where result[i] = [ri, ci] 
//denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic 
//oceans. 
//
// 
// Example 1: 
//
// 
//Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
//
//Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
// 
//
// Example 2: 
//
// 
//Input: heights = [[2,1],[1,2]]
//Output: [[0,0],[0,1],[1,0],[1,1]]
// 
//
// 
// Constraints: 
//
// 
// m == heights.length 
// n == heights[r].length 
// 1 <= m, n <= 200 
// 0 <= heights[r][c] <= 10⁵ 
// 
// Related Topics Array Depth-First Search Breadth-First Search Matrix 👍 2840 ?
//? 681


//leetcode submit region begin(Prohibit modification and deletion)
const DIRECTION: [i32; 5] = [-1, 0, 1, 0, -1];

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_length = heights.len();
        let column_length = heights[0].len();

        let mut p: Vec<Vec<i32>> = vec![vec![0; column_length]; row_length];
        let mut a: Vec<Vec<i32>> = vec![vec![0; column_length]; row_length];

        for i in 0..column_length {
            Self::dfs(&heights, &mut p, 0, i);
            Self::dfs(&heights, &mut a, (row_length - 1), i);
        }
        for i in 0..row_length {
            Self::dfs(&heights, &mut p, i, 0);
            Self::dfs(&heights, &mut a, i, (column_length - 1));
        }

        let mut ret = vec![];
        for r in 0..row_length {
            for c in 0..column_length {
                if p[r][c] == 1 && a[r][c] == 1 && p[r][c] == a[r][c] {
                    ret.push(vec![r as i32, c as i32]);
                }
            }
        }
        ret
    }

    fn dfs(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<i32>>, r: usize, c: usize) {
        if visited[r][c] == 1 {
            return;
        }
        visited[r][c] = 1;
        for i in 0..4 {
            let nr = r as i32 + DIRECTION[i];
            let nc = c as i32 + DIRECTION[i + 1];
            if nr < heights.len() as i32 && nr >= 0 && nc < heights[0].len() as i32 && nc >= 0 {
                if heights[nr as usize][nc as usize] >= heights[r][c] {
                    Self::dfs(heights, visited, nr as usize, nc as usize);
                }
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
        let ret = Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ]);
        assert_eq!(ret, vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ]);
    }
}