// Worst O(nlog n)
// Best O(nlog n)
// Average O(nlog n)
// Space O(n)

fn merge<T: Ord + Copy>(arr: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();
    for v in arr.iter().take(mid + 1).skip(lo) {
        left_half.push(*v);
    }
    for v in arr.iter().take(hi + 1).skip(mid + 1) {
        right_half.push(*v);
    }

    let l_size = left_half.len();
    let r_size = right_half.len();

    let mut l = 0;
    let mut r = 0;
    let mut a = lo;

    while l < l_size && r < r_size {
        if left_half[l] < right_half[r] {
            arr[a] = left_half[l];
            l += 1;
        } else {
            arr[a] = right_half[r];
            r += 1;
        }
        a += 1
    }

    while l < l_size {
        arr[a] = left_half[l];
        l += 1;
        a += 1
    }

    while r < r_size {
        arr[a] = right_half[r];
        r += 1;
        a += 1;
    }
}

fn _merge_sort<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _merge_sort(arr, lo, mid);
        _merge_sort(arr, mid + 1, hi);
        merge(arr, lo, mid, hi);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) -> &[T] {
    let len = arr.len();
    if len > 1 {
        _merge_sort(arr, 0, len - 1);
    }
    return arr;
}