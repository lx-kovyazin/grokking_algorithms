pub mod quick_sort;
pub mod selection_sort;
pub mod bubble_sort;

pub enum Direction {
    Ascending,
    Descending,
}

pub trait Sort<T>
where
    T: PartialOrd,
{
    fn sort(src: &mut [T], direction: Direction);
}
