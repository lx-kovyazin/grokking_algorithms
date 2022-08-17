use crate::helper::slice::Extrema;
use super::{Direction, Sort};

pub struct QuickSort;
impl<T: PartialOrd> Sort<T> for QuickSort {
    fn sort(src: &mut [T], direction: Direction) {
        quick_sort_rec(src, &direction)
    }
}

fn partition_impl<T: PartialOrd>(parts: (&mut [T], &mut T, &mut [T]), direction: &Direction) {
    let (left, pivot, right) = parts;
    let (le, re) = match direction {
        Direction::Ascending => (Extrema::Maxima, Extrema::Minima),
        Direction::Descending => (Extrema::Minima, Extrema::Maxima),
    };
    let lei = le.find_index_by(&left, pivot);
    let rei = re.find_index_by(&right, pivot);
    match (lei, rei) {
        (None, None) => {},
        (Some(lei), None) => std::mem::swap(&mut left[lei], pivot),
        (None, Some(rei)) => std::mem::swap(&mut right[rei], pivot),
        (Some(lei), Some(rei)) => {
            std::mem::swap(&mut left[lei], &mut right[rei]);
            partition_impl((&mut left[1..], pivot, &mut right[1..]), direction);
        }
    }
}

fn partition<T: PartialOrd>(src: &mut [T], direction: &Direction) -> usize {
    use crate::helper::slice::split_at_mut_ex;

    let mid = src.len() / 2;
    partition_impl(split_at_mut_ex(src, mid), direction);
    mid
}

fn quick_sort_rec<T: PartialOrd>(src: &mut [T], direction: &Direction) {
    match src.len() {
        0 | 1 => return,
        _ => {
            let q = partition(src, direction);
            quick_sort_rec(&mut src[..q], direction);
            quick_sort_rec(&mut src[q + 1..], direction);
        }
    }
}


#[test]
fn quick_sort_ascending() {
    let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    QuickSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn quick_sort_descending() {
    let expected = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    QuickSort::sort(&mut unsorted, Direction::Descending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn quick_sort_empty() {
    let expected: [u32; 0] = [];
    let mut unsorted = [];
    QuickSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn quick_sort_single_ascending() {
    let expected = [0];
    let mut unsorted = [0];
    QuickSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn quick_sort_two_ascending() {
    let expected = [0, 1];
    let mut unsorted = [1, 0];
    QuickSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}
