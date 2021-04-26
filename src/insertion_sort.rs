// Worst O(n^2)
// Best O(n)
// Average O(n^2)
// Space O(1)

pub fn insertion_sort(arr: &mut [i32]) -> &[i32] {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
    return arr;
}