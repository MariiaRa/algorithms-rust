use crate::dataStructures::{List, Queue};

mod sorting;
mod searching;
mod dataStructures;

fn main() {
    let mut elements_heapsort = [130, 100, 0, 11, 5, 99];
    println!("The initial array is {:?}", elements_heapsort);
    sorting::heap_sort(&mut elements_heapsort);
    println!("Result of quick sorting: {:?}", elements_heapsort);

    let mut elements_quicksort_one = [13, 10, 2, -31, 0, 16];
    println!("The initial array is {:?}", elements_quicksort_one);
    sorting::quick_sort_one(&mut elements_quicksort_one);
    println!("Result of quick sorting: {:?}", elements_quicksort_one);

    let mut elements_quicksort_two = [4, 65, 2, -31, 0, 99];
    println!("The initial array is {:?}", elements_quicksort_two);
    sorting::quick_sort_two(&mut elements_quicksort_two, &|x,y| x < y);
    println!("Result of quick sorting: {:?}", elements_quicksort_two);

    let mut elements_insert = [13, 10, 33, 9, 56];
    println!("The initial array is {:?}", elements_insert);
    sorting::insertion_sort(&mut elements_insert);
    println!("Result of insertion sorting: {:?}", elements_insert);

    let mut elements_select = [ 9, 4, 8, 3, -5, 2, 1, 6 ];
    println!("The initial array is {:?}", elements_select);
    sorting::selection_sort(&mut elements_select);
    println!("Result of selection sorting: {:?}", elements_select);

    let mut elements_merge = [ 0, 14, 28, 13, -5, 200, 1, 66 ];
    println!("The initial array is {:?}", elements_merge);
    sorting::merge_sort(&mut elements_merge);
    println!("Result of merge sorting: {:?}", elements_merge);

    let mut elements_bubble = [ 0, 44, -5, 20, 11, 6 ];
    println!("The initial array is {:?}", elements_bubble);
    sorting::bubble_sort(&mut elements_bubble);
    println!("Result of merge sorting: {:?}", elements_bubble);
    // Sort strings.
    let mut elements_bubble_string = ["kappa", "gamma", "epsilon", "theta", "beta", "lambda", "alpha"];
    println!("The initial array is {:?}", elements_bubble_string);
    sorting::bubble_sort(&mut elements_bubble_string);
    println!("Result of bubble sorting: {:?}", elements_bubble_string);

    let mut elements_bucket = vec![5, 10, 2, 99, 32, 1, 7, 9, 92, 135, 0, 54];
    println!("The initial array is {:?}", elements_bucket);
    sorting::bucket_sort(&mut elements_bucket, |int| int / 10);
    println!("Result of bucket sorting: {:?}", elements_bucket);

    let elements_count = vec![11, 0, 2, 9, 13, 8, 4, 7, 5, 6];
    println!("The initial array is {:?}", elements_count);
    let result = sorting::counting_sort(elements_count, 0, 13);
    println!("Result of counting sorting: {:?}", &result);

    let mut elements_radix = vec![5, 10, 2, 99, 32, 1, 7, 9, 92, 135, 0, 54];
    println!("The initial array is {:?}", elements_radix);
    sorting::radix_sort_one(&mut elements_radix, 3, 10);
    println!("Result of radix sorting: {:?}", elements_radix);

    let mut elements_shell = [130, 100, 0, 11, 5, 99, 45, 33, 4, 2, 18, 83, 77];
    println!("The initial array is {:?}", elements_shell);
    sorting::shell_sort(&mut elements_shell);
    println!("Result of shell sorting: {:?}", elements_shell);

    let elements =  [130, 100, 0, 11, 5, 99];
    let index_linear_search = searching::linear_search(&elements, &11);
    println!("Array contains number 11 at the index {:?}", index_linear_search);

    let elements_ordered =  [0, 2, 4, 65, 99, 100, 200];
    let index_binary_search = searching::binary_search(&elements_ordered, &2);
    println!("Array contains number 2 at the index {:?}", index_binary_search);

    let elem = 100;
    println!("Sample input list: {:?}", elements_ordered);
    println!("Searched for {} and found index {:?}", elem, searching::ternary_search(&elements_ordered, 0, elements_ordered.len(), elem));

    let mut list = List::new();
    println!("Empty list: {}", list);
    for i in 0..5 {
        list.append(i);
    }
    println!("List with added elements: {}", list);
}
