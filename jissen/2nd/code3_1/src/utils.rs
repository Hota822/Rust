use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;
use crate::SortOrder::{self, *};
use std::cmp::Ordering;



pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    is_sorted(x, Ascending)
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    is_sorted(x, Descending)
}

fn is_sorted<T: Ord>(x: &[T], order: SortOrder) -> bool {
    match order {
        SortOrder::Ascending => x.windows(2).all(|pair| pair[0] <= pair[1]),
        SortOrder::Descending => x.windows(2).all(|pair| pair[0] >= pair[1]),
    }
}



// before refactoring ======================================================
pub fn is_sorted_ascending_before_refactoring<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending_before_refactoring<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}



pub fn new_u32_vec_before_refactoring(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(rng.sample(&Standard));
    }
    v
}