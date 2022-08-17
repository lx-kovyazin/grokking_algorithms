use super::opt::option_add;

pub(crate) fn is_sorted<T: PartialOrd>(src: &[T]) -> bool {
    src.windows(2).all(|w| w[0] <= w[1])
}

#[test]
pub fn is_sorted_fail_for_unsorted() {
    assert!(!is_sorted(&[1, 5, 3]))
}
#[test]
pub fn is_sorted_fail_for_reversed_sorted() {
    assert!(!is_sorted(&[3, 2, 1]));
}

#[test]
pub fn is_sorted_for_empty() {
    assert!(is_sorted::<u32>(&[]));
}

#[test]
pub fn is_sorted_for_single() {
    assert!(is_sorted(&[1]));
}

#[test]
pub fn is_sorted_for_even() {
    assert!(is_sorted(&[1, 2]));
}

#[test]
fn is_sorted_for_odd() {
    assert!(is_sorted(&[1, 2, 3]));
}



pub(crate) enum Extrema {
    Minima,
    Maxima
}

impl Extrema {
    pub(crate) fn find_index_cyc<T: PartialOrd>(&self, src: &[T]) -> Option<usize> {
        if src.len() == 0 {
            return None;
        }

        let pred = match self {
            Extrema::Minima => T::lt,
            Extrema::Maxima => T::gt,
        };

        let mut extrema_index = 0;
        for i in 1..src.len() {
            if pred(&src[i], &src[extrema_index]) {
                extrema_index = i;
            }
        }
        Some(extrema_index)
    }

    pub(crate) fn find_index_rec<T: PartialOrd>(&self, src: &[T]) -> Option<usize> {
        match src.len() {
            0 => return None,
            l => {
                let pred = match self {
                    Extrema::Minima => T::lt,
                    Extrema::Maxima => T::gt,
                };
        
                match option_add(1, self.find_index_rec(&src[1..l])) {
                    Some(i) if pred(&src[i], &src[0]) => Some(i),
                    _ => Some(0)
                }
            }
        }
    }

    pub(crate) fn find_index<T: PartialOrd>(&self, src: &[T]) -> Option<usize> {
        self.find_index_rec(src)
    }

    pub(crate) fn find_index_by<T: PartialOrd>(&self, src: &[T], pivot: &T) -> Option<usize> {
        match src.len() {
            0 => None,
            l => {
                let pred = match self {
                    Extrema::Minima => T::lt,
                    Extrema::Maxima => T::gt,
                };
        
                if pred(&src[0], pivot) {
                    Some(0)
                } else {
                    option_add(1, self.find_index_by(&src[1..l], pivot))
                }
            }
        }
    }
}

#[test]
fn find_minima_index_for_empty_from_0_to_0() {
    assert_eq!(Extrema::Minima.find_index::<u32>(&[]), None);
}

#[test]
fn find_minima_index_for_single_from_0_to_0() {
    assert_eq!(Extrema::Minima.find_index(&[1]), Some(0));
}

#[test]
fn find_minima_index_for_two_from_0_to_1() {
    assert_eq!(Extrema::Minima.find_index(&[1, 2]), Some(0));
}

#[test]
fn find_maxima_index_for_three() {
    assert_eq!(Extrema::Maxima.find_index(&[5, 1, 2]), Some(0));
}

#[test]
fn find_minima_index_for_three_rev_sorted() {
    assert_eq!(Extrema::Minima.find_index(&[5, 2, 1]), Some(2));
}

#[test]
fn find_maxima_index_for_three_dupl() {
    assert_eq!(Extrema::Minima.find_index(&[5, 1, 1]), Some(1));
}

#[test]
fn find_min_index_by() {
    assert_eq!(Extrema::Minima.find_index_by(&[5, 3, 2, 4, 9, 1, 6], &9), Some(0))
}

pub(crate) fn split_at_mut_ex<T>(src: &mut [T], mid: usize) -> (&mut [T], &mut T, &mut [T]) {
    use std::slice::from_raw_parts_mut;
    let len = src.len();
    let ptr = src.as_mut_ptr();
    unsafe {
        (
            from_raw_parts_mut(ptr, mid),
            &mut src[mid],
            from_raw_parts_mut(ptr.add(mid + 1), len - mid - 1)
        )
    }
}