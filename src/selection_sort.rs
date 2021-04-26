// Worst O(n^2)
// Best O(n^2)
// Average O(n^2)
// Space O(1)

pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
    return arr;
}