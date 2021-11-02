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
            // catch an item which &le pivot, should move to left of pivot
            arr[l] = arr[r]; // replace place of l and r
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
        Sort::mergin_sort(arr, l, m, temp);
        Sort::mergin_sort(arr, m, r, temp);
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
}
