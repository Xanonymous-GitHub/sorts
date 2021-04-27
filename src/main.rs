use rand::prelude::*;
use colored::{Colorize, ColoredString};

mod insertion_sort;
mod selection_sort;
mod bubble_sort;
mod shell_sort;
mod heap_sort;

const DATA_LEN: usize = 50000;

fn get_result_text(success: bool) -> ColoredString {
    if success {
        "PASSED".green()
    } else {
        "FAILED".red()
    }
}

fn get_rand_arr<const SIZE: usize>(rng: &mut ThreadRng) -> [i32; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(i32::MIN..i32::MAX);
    }
    arr
}

fn main() {
    let mut rng = rand::thread_rng();
    let test_array: [i32; DATA_LEN] = get_rand_arr(&mut rng);
    let mut sorted_test_array = test_array.clone();
    sorted_test_array.sort();

    // insertion sort
    let mut unsorted = test_array.clone();
    let insertion_sort_result = insertion_sort::insertion_sort(&mut unsorted);
    println!("Insertion sort {}", get_result_text(insertion_sort_result == sorted_test_array).green());
    assert_eq!(insertion_sort_result, sorted_test_array);

    // selection sort
    let mut unsorted = test_array.clone();
    let selection_sort_result = selection_sort::selection_sort(&mut unsorted);
    println!("selection sort {}", get_result_text(selection_sort_result == sorted_test_array));
    assert_eq!(selection_sort_result, sorted_test_array);

    // bubble sort
    let mut unsorted = test_array.clone();
    let bubble_sort_result = bubble_sort::bubble_sort(&mut unsorted);
    println!("bubble sort {}", get_result_text(bubble_sort_result == sorted_test_array));
    assert_eq!(bubble_sort_result, sorted_test_array);

    // shell sort
    let mut unsorted = test_array.clone();
    let shell_sort_result = shell_sort::shell_sort(&mut unsorted);
    println!("shell sort {}", get_result_text(shell_sort_result == sorted_test_array));
    assert_eq!(shell_sort_result, sorted_test_array);

    // heap sort
    let mut unsorted = test_array.clone();
    let heap_sort_result = heap_sort::heap_sort(&mut unsorted);
    println!("heap sort {}", get_result_text(heap_sort_result == sorted_test_array));
    assert_eq!(heap_sort_result, sorted_test_array);
}
