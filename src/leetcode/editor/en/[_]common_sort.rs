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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr = vec![6, 1, 4, 5, 2];
        Sort::quick_sort(&mut arr, 0, 5);
        println!("{:?}", &arr);
        assert_eq!(
            arr[0],
            1
        );
        assert_eq!(
            arr[1],
            2
        );
        assert_eq!(
            arr[2],
            4
        );
        assert_eq!(
            arr[3],
            5
        );
        assert_eq!(
            arr[4],
            6
        );
    }
}
