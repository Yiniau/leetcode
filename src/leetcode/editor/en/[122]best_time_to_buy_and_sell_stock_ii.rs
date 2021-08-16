//noinspection ALL
#[allow(dead_code)]
pub struct Solution {}

//You are given an array prices where prices[i] is the price of a given stock on
// the ith day. 
//
// Find the maximum profit you can achieve. You may complete as many transaction
//s as you like (i.e., buy one and sell one share of the stock multiple times). 
//
// Note: You may not engage in multiple transactions simultaneously (i.e., you m
//ust sell the stock before you buy again). 
//
// 
// Example 1: 
//
// 
//Input: prices = [7,1,5,3,6,4]
//Output: 7
//Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 
//5-1 = 4.
//Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
//
// 
//
// Example 2: 
//
// 
//Input: prices = [1,2,3,4,5]
//Output: 4
//Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 
//5-1 = 4.
//Note that you cannot buy on day 1, buy on day 2 and sell them later, as you ar
//e engaging multiple transactions at the same time. You must sell before buying a
//gain.
// 
//
// Example 3: 
//
// 
//Input: prices = [7,6,4,3,1]
//Output: 0
//Explanation: In this case, no transaction is done, i.e., max profit = 0.
// 
//
// 
// Constraints: 
//
// 
// 1 <= prices.length <= 3 * 104 
// 0 <= prices[i] <= 104 
// 
// Related Topics Array Dynamic Programming Greedy 
// 👍 4672 👎 2097



//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ret = 0;

        let len = prices.len();
        let mut i = 1;
        while i < len {
            let distance = prices[i] - prices[i - 1];
            if distance > 0 {
                ret += distance;
            }
            i += 1;
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
