use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp;

// pseudo-code for traditional solution
/* for i from 1 to length[A]-1 do
   value := A[i]
   j := i-1
   while j >= 0 and A[j] > value do
     A[j+1] := A[j]
     j := j-1
   done
   A[j+1] = value
 done*/

pub fn insertion_sort<T: Ord>(elements: &mut [T]) {
    for i in 1..elements.len() {
        let mut j = i;
        while j > 0 && elements[j] < elements[j - 1] {
            elements.swap(j, j - 1);
            j = j - 1;
        }
    }
}
// pseudo-code for traditional solution
/*for i = 1 to n − 1 do
  min = i
    for j = i + 1 to n do
    // Find the index of the ith smallest element
      if A[j] < A[min] then
        min = j
      end if
    end for
   Swap A[min] and A[i]
end for*/

pub fn selection_sort<T: Ord>(elements: &mut [T]) {
    let mut min;
    for i in 0..elements.len() - 1 {
        min = i;
        for j in (i + 1)..elements.len() {
            if elements[j] < elements[min] {
                min = j;
            }
        }
        elements.swap(min, i);
        // swapping
        // let tmp = array[i];
        // array[i] = array[min];
        // array[min] = tmp;
    }
}

/*function merge(left,right)
   var list result
   while length(left) > 0 and length(right) > 0
       if first(left) ≤ first(right)
           append first(left) to result
           left = rest(left)
       else
           append first(right) to result
           right = rest(right)
   if length(left) > 0
       append rest(left) to result
   if length(right) > 0
       append rest(right) to result
   return result*/

fn merge<T: Copy + PartialOrd>(left: &[T], right: &[T], sorted: &mut [T]) {
    assert_eq!(left.len() + right.len(), sorted.len());
    let mut l = 0;
    let mut r = 0;
    let mut k = 0;
    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            sorted[k] = left[l];
            k += 1;
            l += 1;
        } else {
            sorted[k] = right[r];
            k += 1;
            r += 1;
        }
    }
    if l < left.len() {
        sorted[k..].copy_from_slice(&left[l..]);
    }
    if r < right.len() {
        sorted[k..].copy_from_slice(&right[r..]);
    }
}

pub fn merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    let n = arr.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut arr[0..m]);
    merge_sort(&mut arr[m..n]);

    let mut sorted: Vec<T> = arr.to_vec();

    merge(&arr[0..m], &arr[m..n], &mut sorted[..]);

    arr.copy_from_slice(&sorted);
}

pub fn bubble_sort<T: Ord>(elements: &mut [T]) {
    let mut n = elements.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if elements[i - 1] > elements[i] {
                elements.swap(i - 1, i);
                swapped = true;
            }
        }
        n = n - 1;
    }
}

/*function quicksort(array)
if length(array) > 1
  pivot := select any element of array
  left := first index of array
  right := last index of array

  while left ≤ right
    while array[left] < pivot
      left := left + 1
    while array[right] > pivot
      right := right - 1
  if left ≤ right
    swap array[left] with array[right]
    left := left + 1
    right := right - 1

  quicksort(array from first index to right)
  quicksort(array from left to last index)*/

pub fn quick_sort_one<T: Ord>(elements: &mut [T]) {
    quicksort_by(elements, &|x, y| x.cmp(y))
}

fn quicksort_by<T, F>(elements: &mut [T], compare_function: &F)
    where F: Fn(&T, &T) -> Ordering {
    let len: usize = elements.len();
    if len <= 1 {
        return;
    }

    let pivot: usize = 0;
    elements.swap(pivot, len / 2);
    let mut left: usize = 1;
    let mut right: usize = elements.len() - 1;

    loop {
        while left < len && compare_function(&elements[left], &elements[pivot]) != Ordering::Greater {
            left += 1
        }
        while right > 0 && compare_function(&elements[right], &elements[pivot]) != Ordering::Less {
            right -= 1
        }
        if left >= right {
            break;
        }
        elements.swap(left, right);
        left += 1;
        right -= 1;
    }
    elements.swap(pivot, right);
    quicksort_by(&mut elements[0..cmp::min(left - 1, right)], compare_function);
    quicksort_by(&mut elements[cmp::max(left, right + 1)..], compare_function);
}

pub fn quick_sort_two<T, F>(elements: &mut [T], f: &F)
    where F: Fn(&T, &T) -> bool {
    let len = elements.len();
    if len >= 2 {
        let pivot_index = partition_pivot_right(elements, f);
        quick_sort_two(&mut elements[0..pivot_index], f);
        quick_sort_two(&mut elements[pivot_index + 1..len], f);
    }
}

fn partition_pivot_right<T, F>(elements: &mut [T], f: &F) -> usize
    where F: Fn(&T, &T) -> bool {
    let len = elements.len();
    let pivot_index = len - 1; // pick last element as pivot
    let left = 0; // starting index
    let right = len - 1; // ending index

    let mut store_index = 0;
    for i in left..right {
        if f(&elements[i], &elements[pivot_index]) {
            elements.swap(store_index, i);
            store_index += 1;
        }
    }

    elements.swap(store_index, pivot_index); //place pivot between left and right
    store_index
}

fn partition_pivot_left<T, F>(elements: &mut [T], f: &F) -> usize
    where F: Fn(&T, &T) -> bool {
    let pivot_index = 0; // pick first element as pivot
    let left = 0; // starting index
    let right = elements.len() - 1; // ending index

    let mut store_index = 0;

    //we start from the leftmost element and keep track of index of smaller (or equal to) elements as store_index.
    // While traversing, if we find a smaller element, we swap current element with arr[store_index].
    // Otherwise we ignore current element.

    for i in left..right {
        if f(&elements[i], &elements[pivot_index]) {
            elements.swap(store_index, i);
            store_index += 1;
        }
    }

    elements.swap(store_index, pivot_index); //we place pivot at correct position between left and right
    store_index
}

// To heapify a subtree rooted with node i which is
// an index in arr[]. n is size of heap
fn heapify<T: Ord>(elements: &mut [T], root: usize, size: usize) {
    let mut parent = root;

    loop {
        let largest = parent;
        // If left child is larger than root
        let left = 2 * largest + 1;
        if left < size && elements[parent] < elements[left] {
            parent = left
        }
        // If right child is larger than largest so far
        let right = 2 * largest + 2;
        if right < size && elements[parent] < elements[right] {
            parent = right
        }

        elements.swap(largest, parent);

        if largest == parent {
            break;
        }
    }
}

pub fn heap_sort<T: Ord>(elements: &mut [T]) {
    let len = elements.len();

    // Build heap (rearrange array)
    for i in (0..len / 2 + 1).rev() {
        heapify(elements, i, len);
    }
    // call max heapify on the reduced heap
    for i in (1..len).rev() {
        elements.swap(0, i);
        heapify(elements, 0, i - 1);
    }
}

/// Bucket to store elements.
struct Bucket<H, T> {
    hash: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    /// Create a new bucket and insert its first value.
    ///
    /// * `hash` - Hash value generated by hasher param of `bucket_sort`.
    /// * `value` - Value to be put in the bucket.
    pub fn new(hash: H, value: T) -> Bucket<H, T> {
        Bucket {
            hash: hash,
            values: vec![value],
        }
    }
}

pub fn bucket_sort<H, F, T>(elements: &mut Vec<T>, hasher: F)
    where H: Ord,
          F: Fn(&T) -> H,
          T: Ord + Clone, {

    // 1. Create buckets.
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    // 2. Scatter
    for element in elements.iter() {
        let hash = hasher(&element); // 2.1.

        let value = element.clone();
        // 2.2.
        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            // If exists, push the value to the bucket.
            Ok(index) => buckets[index].values.push(value),
            // If none, create and new bucket and insert value in.
            Err(index) => buckets.insert(index, Bucket::new(hash, value)),
        }
    }

    // 3. Inner sort and gather
    let ret = buckets.into_iter().flat_map(|mut bucket| {
        bucket.values.sort(); // 3.1.
        bucket.values
    }).collect::<Vec<T>>();   // 3.2.

    elements.clone_from_slice(&ret); // 4 Copy to original array
}

pub fn counting_sort(
    mut elements: Vec<usize>,
    min: usize,
    max: usize,
) -> Vec<usize> {

    // create and fill counting bucket with 0
    let max = elements
        .iter()
        .max()
        .expect("Oops, vector was empty")
        .to_owned();

    let mut count: Vec<usize> = vec![0; max+1];

    // create and fill result vector with 0
    let mut result: Vec<usize>= vec![0; elements.len()];

    for integer in &elements {
        count[integer - min] = count[integer - min] + 1;
    }

    for i in 1..max-min + 1 {
        count[i] += count[i-1];
    }

    for elem in elements.iter().rev() {
        count[elem - min] -= 1;
        result[count[elem - min]] = elem.to_owned();
    }
    result
}

fn radix_count_sort(elements: &mut Vec<usize>, digit: usize, base: usize) {

    let mut count: Vec<usize> = vec![0; base];

    // create and fill result vector with 0
    let mut result: Vec<usize>= vec![0; elements.len()];

    for integer in elements.iter() {
        count[(integer / digit % base) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i-1];
    }

    for integer in elements.iter().rev() {
        count[(integer/digit)%base] -= 1;
        result[count[(integer/digit)%base]] = integer.to_owned();
    }

    elements.clone_from_slice(&result);
}

pub fn radix_sort_one(elements: &mut  Vec<usize>, number_of_digits: usize, base: usize) {
    let mut digit = 1;
    for i in 1..number_of_digits+1 {
        radix_count_sort(elements, digit, base);
        digit *= base;
    }
}

pub fn counting_sort_two<F, T>(arr: &mut [T], min: usize, max: usize, key: F)
    where F: Fn(&T) -> usize,
          T: Clone, {
    let mut prefix_sums = {
        // 1. Initialize the count array with default value 0.
        let len = max - min;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        // 2. Scan elements to collect counts.
        for value in arr.iter() {
            count_arr[key(value)] += 1;
        }

        // 3. Calculate prefix sum.
        count_arr.into_iter().scan(0, |state, x| {
            *state += x;
            Some(*state - x)
        }).collect::<Vec<usize>>()
    };

    // 4. Use prefix sum as index position of output element.
    for value in arr.to_vec().iter() {
        let index = key(value);
        arr[prefix_sums[index]] = value.clone();
        prefix_sums[index] += 1;
    }
}

pub fn radix_sort_two(arr: &mut [i32]) {
    let radix = 10;
    let mut digit = 1;
    let max_value = arr
        .iter()
        .max()
        .unwrap_or(&0)
        .clone();
    while digit <= max_value {
        counting_sort_two(arr, 0, 9, |t| (t / digit % radix) as usize); // 5
        digit *= radix;
    }
}

/// Marcin Ciura's gap sequence.
const MARCIN_GAPS: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];

pub fn shell_sort(arr: &mut [i32]) {
    let len = arr.len();
    for gap in MARCIN_GAPS.iter() {
        let mut i = *gap;
        while i < len {
            let mut j = i;
            while j >= *gap && arr[j - gap] > arr[j] {
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1;
        }
    }
}

