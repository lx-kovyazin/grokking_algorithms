#![feature(test)]
#![feature(slice_partition_dedup)]
#![feature(generic_arg_infer)]
extern crate grokking_algorithms;
extern crate test;

use grokking_algorithms::algorithm::sort::{
    bubble_sort::BubbleSort, quick_sort::QuickSort, selection_sort::SelectionSort, Direction, Sort,
};
use test::Bencher;

fn sort_bencher<S, T, const N: usize>(b: &mut Bencher, mut unsorted: [T; N])
where
    T: PartialOrd,
    S: Sort<T>,
{
    let (mut dedup, _) = unsorted.partition_dedup();
    b.iter(|| S::sort(&mut dedup, Direction::Ascending))
}

fn sort_bencher_for_small<S: Sort<u32>>(b: &mut Bencher) {
    const SMALL_UNSORTED_ARRAY: [u32; 10] = [8, 2, 1, 3, 5, 4, 9, 0, 7, 6];
    sort_bencher::<S, _, _>(b, SMALL_UNSORTED_ARRAY)
}

fn sort_bencher_for_large<S: Sort<u32>>(b: &mut Bencher) {
    const LARGE_UNSORTED_ARRAY: [u32; 100] = [
        7, 5446221, 96058, 9448, 681319, 3509, 7337301, 6087, 47, 1, 600752, 13689141, 640927,
        250955893, 1963543, 4232714, 1395, 5, 60893, 542, 426, 54002, 585, 4530715, 3351343, 757,
        37, 2572127, 477230, 5, 544, 3, 9, 62, 976, 293, 23, 2183, 452, 89535, 2127044, 6516186,
        345, 44, 18609434, 9980894, 791800, 3826, 131846237, 3424, 3368280, 731976740, 1, 932,
        333610, 567097684, 33, 62906749, 25376, 36999678, 9662, 80502, 819383, 945363601, 9268940,
        15, 98898, 2377729, 148961, 9043910, 46049685, 9, 63, 5276, 8312, 574389122, 294, 7178679,
        492, 4, 74252401, 0, 23750207, 821545723, 71441, 95752226, 723, 983295, 83, 246067638, 45,
        7, 1367, 52540, 194189, 76, 6788800, 13921, 950226, 9758,
    ];

    sort_bencher::<S, _, _>(b, LARGE_UNSORTED_ARRAY)
}

#[bench]
fn quick_sort_ascending(b: &mut Bencher) {
    sort_bencher_for_small::<QuickSort>(b)
}

#[bench]
fn selection_sort_ascending(b: &mut Bencher) {
    sort_bencher_for_small::<SelectionSort>(b)
}

#[bench]
fn bubble_sort_ascending(b: &mut Bencher) {
    sort_bencher_for_small::<BubbleSort>(b)
}

#[bench]
fn quick_sort_ascending_large(b: &mut Bencher) {
    sort_bencher_for_large::<QuickSort>(b)
}

#[bench]
fn selection_sort_ascending_large(b: &mut Bencher) {
    sort_bencher_for_large::<SelectionSort>(b)
}

#[bench]
fn bubble_sort_ascending_large(b: &mut Bencher) {
    sort_bencher_for_large::<BubbleSort>(b)
}
