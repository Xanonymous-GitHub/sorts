// Worst O(n^2)
// Best O(n)
// Average O(n^2)
// Space O(1)

pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
    return arr;
}