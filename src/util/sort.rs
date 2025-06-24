fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];

    let mut i = 0;

    for j in 0..arr.len() {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    i
}

pub fn qs(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);

    qs(&mut arr[..pivot - 1]);
    qs(&mut arr[pivot..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut array1 = [3, 4, 2, 1, 3, 3];
        let mut array2 = [4, 3, 5, 3, 9, 3];
        qs(&mut array1);
        qs(&mut array2);
        assert_eq!(array1, [1, 2, 3, 3, 3, 4]);
        assert_eq!(array2, [3, 3, 3, 4, 5, 9]);
    }
}
