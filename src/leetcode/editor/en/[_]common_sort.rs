#[allow(dead_code)]
pub struct Sort {}

impl Sort {
    pub fn quick_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
        let mut l = start;
        let mut r = end - 1;
        if l >= r {
            return;
        }
        let pivot = arr[l];
        while l < r {
            // scan items at right of pivot
            while l < r && arr[r] >= pivot {
                r -= 1;
            }
            // catch an item which &ge pivot, should move to left of pivot
            arr[l] = arr[r]; // swap of l and r
                             // scan left
            while l < r && arr[l] <= pivot {
                l += 1;
            }
            arr[r] = arr[l];
        }
        arr[l] = pivot;
        Sort::quick_sort(arr, start, l);
        Sort::quick_sort(arr, l + 1, end);
    }

    pub fn merge_sort(arr: &mut Vec<i32>, l: usize, r: usize, temp: &mut Vec<i32>) {
        if l + 1 >= r {
            // => 1 >= r - l
            // => r - l <= 1
            // => make sure [l..r).len() <= 1;
            return;
        }
        // [l..m..r), left close, right open
        // because l + 1 < r
        // => r > l + 1
        // => l + r - 1 > 2l
        // => m > l
        let m = l + (r - l) / 2; // divide
        Self::merge_sort(arr, l, m, temp);
        Self::merge_sort(arr, m, r, temp);
        // we can sure [l..m).len() <= 1 and [m..r).len() <= 1;
        // Only starting from a single element can make sure the orderliness of both sides of array to be merged

        let mut ml = l;
        let mut mr = m;
        let mut i = l;
        // rolling merge start from left
        while ml < m || mr < r {
            // avoid mr or ml always grow
            if (mr >= r) || (ml < m && arr[ml] <= arr[mr]) {
                temp[i] = arr[ml];
                ml += 1;
            } else {
                temp[i] = arr[mr];
                mr += 1;
            }
            i += 1;
        }
        for t in 0..arr.len() {
            arr[t] = temp[t];
        }
        // temp.clear();
    }

    pub fn bubble_sort(nums: &mut Vec<i32>, n: usize) {
        let mut swapped = false;
        // every turn, the largest item will be moved to array end
        for i in 1..n {
            swapped = false;
            for j in 1..(n - i + 1) {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j-1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut arr = vec![6, 1, 4, 5, 2];
        Sort::quick_sort(&mut arr, 0, 5);
        println!("{:?}", &arr);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 4);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[4], 6);
    }

    #[test]
    fn merge_sort_test() {
        let mut arr = vec![6, 1, 4, 5, 2];
        let mut temp = arr.clone();
        let len = arr.len();
        Sort::merge_sort(&mut arr, 0, len, &mut temp);
        println!("{:?}", &arr);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 4);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[4], 6);
    }

    #[test]
    fn bubble_sort_test() {
        let mut arr = vec![6, 1, 4, 5, 2];
        let len = arr.len();
        Sort::bubble_sort(&mut arr, len);
        println!("{:?}", &arr);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 4);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[4], 6);

        let mut arr2 = vec![6, 5, 4, 3, 2, 1];
        let len2 = arr2.len();
        Sort::bubble_sort(&mut arr2, len2);
        println!("{:?}", &arr2);
        assert_eq!(arr2[0], 1);
        assert_eq!(arr2[1], 2);
        assert_eq!(arr2[2], 3);
        assert_eq!(arr2[3], 4);
        assert_eq!(arr2[4], 5);
        assert_eq!(arr2[5], 6);
    }
}
