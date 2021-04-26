// Worst O(n^2) ~ O(nlog^2n) (Depends on sequence)
// Best O(nlog n)
// Average Depends on sequence
// Space O(1)

pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) -> &[T] {
    fn insertion<T: Ord + Copy>(arr: &mut [T], start: usize, gap: usize) {
        for i in ((start + gap)..arr.len()).step_by(gap) {
            let val_current = arr[i];
            let mut pos = i;
            while pos >= gap && arr[pos - gap] > val_current {
                arr[pos] = arr[pos - gap];
                pos -= gap;
            }
            arr[pos] = val_current;
        }
    }

    let mut count_sublist = arr.len() / 2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(arr, pos_start, count_sublist);
        }
        count_sublist /= 2;
    }

    return arr;
}