use std::cmp::Ord;
use std::cmp::Ordering::*;

pub fn linear_search<T>(elements: &[T], target: &T) -> Option<usize> where
    T: PartialEq {
    for (index, item) in elements.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

pub fn binary_search<T: Ord>(elements: &[T], elem: &T) -> Option<usize> {
    let mut size = elements.len();
    let mut base = 0;

    while size > 0 {
        size /= 2;
        let mid = base + size;
        base = match elements[mid].cmp(elem) {
            Less    => mid,
            Greater => base,
            Equal   => return Some(mid)
        };
    }
    None
}

pub fn ternary_search<T>(elements: &[T], start: usize, end: usize, elem: T) -> Option<usize>
    where
        T: PartialOrd {
    if start <= end {
        let middle1 = start + (end - start) / 3;
        let middle2 = end - (end - start) / 3;

        if elements[middle1] == elem {
            return Some(middle1);
        }
        if elements[middle2] == elem {
            return Some(middle2);
        }
        if elem < elements[middle1] {
            return ternary_search(&elements, start, middle1, elem);
        }
        else if elem > elements[middle1] && elem < elements[middle2] {
            return ternary_search(&elements, middle1, middle2, elem);
        }
        else {
            return ternary_search(&elements, middle2, end, elem);
        }
    }
    None
}