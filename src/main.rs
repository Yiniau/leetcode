#[allow(dead_code)]

#[path = "./leetcode/editor/en/[122]best_time_to_buy_and_sell_stock_ii.rs"]
mod best_time_to_buy_and_sell_stock_ii;
#[path = "./leetcode/editor/en/[406]queue_reconstruction_by_height.rs"]
mod queue_reconstruction_by_height;
#[path = "./leetcode/editor/en/[665]non_decreasing_array.rs"]
mod non_decreasing_array;
#[path = "./leetcode/editor/en/[763]partition_labels.rs"]
mod partition_labels;
#[path = "./leetcode/editor/en/[88]merge_sorted_array.rs"]
mod merge_sorted_array;
#[path = "./leetcode/editor/en/[76]minimum_window_substring.rs"]
mod minimum_window_substring;
#[path = "./leetcode/editor/en/[633]sum_of_square_numbers.rs"]
mod sum_of_square_numbers;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_time_to_buy_and_sell_stock_ii() {
        assert_eq!(
            best_time_to_buy_and_sell_stock_ii::Solution::max_profit(vec![1, 2, 3, 4, 5]),
            4
        );
        assert_eq!(
            best_time_to_buy_and_sell_stock_ii::Solution::max_profit(vec![7, 6, 4, 3, 1]),
            0
        );
    }
}
