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
#[path = "./leetcode/editor/en/[680]valid_palindrome_ii.rs"]
mod valid_palindrome_ii;
#[path = "./leetcode/editor/en/[524]_longest_word_in_dictionary_through_deleting.rs"]
mod longest_word_in_dictionary_through_deleting;
#[path = "leetcode/editor/en/[340]longest_substring_with_at_most_k_distinct.rs"]
mod longest_substring_with_at_most_k_distinct;
#[path = "leetcode/editor/en/[69]_sqrt(x).rs"]
mod _sqrt;
#[path = "leetcode/editor/en/[34]_find_first_and_last_position_of_element_in_sorted_array.rs"]
mod _find_first_and_last_position_of_element_in_sorted_array;
#[path = "leetcode/editor/en/[81]_search_in_rotated_sorted_array_i_i.rs"]
mod _search_in_rotated_sorted_array_i_i;

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
