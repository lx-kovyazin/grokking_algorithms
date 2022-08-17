use crate::helper::{slice::is_sorted, opt::option_add};

fn binary_search_cyc<T: PartialOrd>(src: &[T], item: T) -> Option<usize> {
    let mut left = 0;
    let (mut right, ovf) = src.len().overflowing_sub(1);
    while !ovf && left <= right {
        let mid = (left + right) / 2;
        if item < src[mid] {
            right = mid - 1;
        } else if src[mid] < item {
            left = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

fn binary_search_rec<T: PartialOrd>(src: &[T], item: T) -> Option<usize> {
    match src.len() {
        0 => None,
        l => {
            let mid = l / 2;
            match &src[mid] {
                guess if item.lt(guess) => binary_search_rec(&src[0..mid], item),
                guess if item.gt(guess) => {
                    let left = mid + 1;
                    option_add(left, binary_search_rec(&src[left..l], item))
                },
                _ => Some(mid)
            }
        }
    }
}

pub fn binary_search<T: PartialOrd>(src: &[T], item: T) -> Option<usize> {
    assert!(is_sorted(src), "{} must be sorted.", stringify!(src));
    binary_search_rec(src, item)
}

#[test]
fn binary_search_for_empty_is_none() {
    assert_eq!(binary_search::<u32>(&[], 5), None);
}

#[test]
fn binary_search_for_not_included_item_is_none() {
    assert_eq!(binary_search(&[1], 2), None);
}

#[test]
fn binary_search_for_single() {
    assert_eq!(binary_search(&[1], 1), Some(0));
}

#[test]
fn binary_search_for_two() {
    assert_eq!(binary_search(&[1, 2], 2), Some(1));
}

#[test]
fn binary_search_at_mid() {
    assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5), Some(4));
}

#[test]
fn binary_search_at_left() {
    assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 1), Some(0));
}

#[test]
fn binary_search_at_left_part() {
    assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3), Some(2));
}

#[test]
fn binary_search_at_right() {
    assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 9), Some(8));
}

#[test]
fn binary_search_at_right_part() {
    assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 8), Some(7));
}

#[test]
#[should_panic]
fn binary_search_panic_on_unsorted() {
    binary_search(&[2, 1], 2);
}
