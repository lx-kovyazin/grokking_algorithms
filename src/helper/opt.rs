use std::ops::Add;

pub(crate) fn option_add<T: Add<Output = T>>(lhs: T, rhs: Option<T>) -> Option<T> {
    rhs.and_then(|rhs| Some(lhs + rhs))
}

#[test]
fn option_add_test() {
    assert_eq!(option_add(5, Some(7)), Some(12));
    assert_eq!(option_add(5, None), None);
}