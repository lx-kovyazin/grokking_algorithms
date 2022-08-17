use super::{Sort, Direction};

pub struct BubbleSort;
impl<T: PartialOrd> Sort<T> for BubbleSort {
    fn sort(src: &mut [T], direction: Direction) {
        bubble_sort_rec(src, direction)
    }
}

fn bubble_sort_rec<T: PartialOrd>(src: &mut [T], direction: Direction) {
    match src.len() {
        0 => {},
        l => {
            let pred = match direction {
                Direction::Ascending => T::lt,
                Direction::Descending => T::gt,
            };
            for li in 0..l - 1 {
                let ri = li + 1;
                if pred(&src[ri], &src[li]) {
                    src.swap(ri, li)
                }
            }
            bubble_sort_rec(&mut src[0..l - 1], direction)
        }
    }
}


#[test]
fn bubble_sort_ascending() {
    let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    BubbleSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn bubble_sort_descending() {
    let expected = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut unsorted = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    BubbleSort::sort(&mut unsorted, Direction::Descending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn bubble_sort_empty() {
    let expected: [u32; 0] = [];
    let mut unsorted = [];
    BubbleSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn bubble_sort_single_ascending() {
    let expected = [0];
    let mut unsorted = [0];
    BubbleSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}

#[test]
fn bubble_sort_two_ascending() {
    let expected = [0, 1];
    let mut unsorted = [1, 0];
    BubbleSort::sort(&mut unsorted, Direction::Ascending);
    assert!(expected.eq(&unsorted))
}
