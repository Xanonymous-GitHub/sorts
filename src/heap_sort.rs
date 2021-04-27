// Worst O(nlog n)
// Best O(nlog n)
// Average O(nlog n)
// Space O(1)

fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };
        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 1) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}


pub fn heap_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    if arr.len() <= 1 {
        return arr;
    }
    heapify(arr);
    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
    return arr;
}