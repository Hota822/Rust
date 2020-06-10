use crate::models::*;

use SortOrder::{ Ascending as Asc, Descending as Desc };
use rand::{ Rng, SeedableRng };
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;


// pub fn new_u32_vec(n: usize) -> Vec<u32> {
//     let mut rng = Pcg64Mcg::from_seed([0; 16]);
//     let mut v = Vec::with_capacity(n);

//     for _ in 0..n {
//         v.push(rng.sample(&Standard))
//     }
//     v
// }


pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    do_sort(x, Asc)
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    do_sort(x, Desc)
}

fn do_sort<T: Ord>(x: &[T], order: SortOrder) -> bool {
    match order {
        Asc => x.windows(2).all(|pair| pair[0] <= pair[1]),
        Desc => x.windows(2).all(|pair| pair[0] >= pair[1]),
    }
}