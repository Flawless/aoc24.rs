/// Binary search implementation
/// Returns the index of the first occurrence of the target element in the array if found, otherwise None
pub fn bs(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mut mid = left + (right - left) / 2;

        if arr[mid] == target {
            while mid > 0 && arr[mid - 1] == target {
                mid -= 1;
            }
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let array = [1, 2, 3, 4, 5, 6];
        let target = 3;
        let index = bs(&array, target);
        assert_eq!(index, Some(2));

        let array = [1, 3, 3, 4, 5, 6];
        let target = 3;
        let index = bs(&array, target);
        assert_eq!(index, Some(1));

        let array = [1, 2, 4, 5, 6];
        let target = 3;
        let index = bs(&array, target);
        assert_eq!(index, None);
    }
}
