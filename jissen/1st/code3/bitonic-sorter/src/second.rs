use crate::models::*;
use SortOrder::{ Ascending as Asc, Descending as Desc };

pub fn sort<T>(x: &mut [T], order: &SortOrder) -> Result<(), String>
where
    T: Ord
{
    if x.len().is_power_of_two() {
        match *order {
            Asc => do_sort(x, true),
            Desc => do_sort(x, false),
        };
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {}", x.len()))
    }
}

fn do_sort<T>(x: &mut [T], up: bool)
where
    T: Ord
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }
}


fn sub_sort<T>(x: &mut [T], up: bool)
where
    T: Ord
{
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up)
    }
}

fn compare_and_swap<T>(x: &mut [T], up: bool)
where
    T: Ord
{
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    use crate::models::*;
    use SortOrder::{ Ascending as Asc, Descending as Desc };
    #[test]
    fn sort_u32_ascending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert!(sort(&mut x, &Asc).is_ok());
        assert_eq!(x, [4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert!(sort(&mut x, &Desc).is_ok());
        assert_eq!(x, [330, 110, 30, 21, 20, 11, 10, 4, ]);
    }

    #[test]
    fn sort_string_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert!(sort(&mut x, &Asc).is_ok());
        assert_eq!(x, ["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with",]);
    }

    #[test]
    fn sort_string_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert!(sort(&mut x, &Desc).is_ok());
        assert_eq!(x, ["with", "no", "memory-efficient", "is", "fast", "and", "Rust","GC"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Asc).is_err())
    }
}