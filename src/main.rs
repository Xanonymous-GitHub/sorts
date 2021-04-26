use rand::prelude::*;
use colored::{Colorize, ColoredString};

mod insertion_sort;
mod selection_sort;

fn get_result_text(success: bool) -> ColoredString {
    if success {
        "PASSED".green()
    } else {
        "FAILED".red()
    }
}

fn get_rand_arr<const SIZE: usize>(rng: &mut ThreadRng, n: i32) -> [i32; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(-n..n);
    }
    arr
}

fn main() {
    let mut rng = rand::thread_rng();
    let test_array: [i32; 10000] = get_rand_arr(&mut rng, 100000);
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
}
