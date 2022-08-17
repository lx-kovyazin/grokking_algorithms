use crate::helper::slice::Extrema;
use super::{Direction, Sort};

pub struct SelectionSort;
impl<T: PartialOrd> Sort<T> for SelectionSort {
    fn sort(src: &mut [T], direction: Direction) {
        selection_sort_rec(src, direction)
    }
}

fn selection_sort_cyc<T: PartialOrd>(src: &mut [T], direction: Direction) {
    let l = src.len();
    if l == 0 {
        return;
    }

    let extrema = match direction {
        Direction::Ascending => Extrema::Minima,
        Direction::Descending => Extrema::Maxima,
    };

    for i in 0..(l-1) {
        let e = match extrema.find_index(&src[i..l]) {
            None => return,
            Some(e) => e + i,
        };
        if i != e {
            src.swap(i, e)
        }
    }
}

fn selection_sort_rec<T: PartialOrd>(src: &mut [T], direction: Direction) {
    match src.len() {
        0 => return,
        l => {
            let extrema = match direction {
                Direction::Ascending => Extrema::Minima,
                Direction::Descending => Extrema::Maxima,
            };
            
            let e = extrema.find_index(&src[0..l]).unwrap();
            if 0 != e {
                src.swap(0, e)
            }
            selection_sort_rec(&mut src[1..l], direction)
        }
    }
}

#[test]
fn selection_sort_ascending() {
    let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    SelectionSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn selection_sort_descending() {
    let expected = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    SelectionSort::sort(&mut unsorted, Direction::Descending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn selection_sort_empty() {
    let expected: [u32; 0] = [];
    let mut unsorted = [];
    SelectionSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn selection_sort_single_ascending() {
    let expected = [0];
    let mut unsorted = [0];
    SelectionSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn selection_sort_two_ascending() {
    let expected = [0, 1];
    let mut unsorted = [1, 0];
    SelectionSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}
