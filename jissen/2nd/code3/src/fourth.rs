use crate::lib::SortOrder;
use rayon;
use std::cmp::Ordering;

const PARALLEL_THRESHOLD: usize = 4096;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where T: Send,
      F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two."))
    }

}

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where T: Send,
      F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        if mid_point > PARALLEL_THRESHOLD {
            let (first, second) =  x.split_at_mut(mid_point);
            rayon::join(|| do_sort(first, true, comparator),
                        || do_sort(second, false, comparator));
        } else {
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where T: Send,
      F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        if mid_point > PARALLEL_THRESHOLD {
            let (first, second) = x.split_at_mut(mid_point);
            rayon::join(|| sub_sort(first, forward, comparator),
                        || sub_sort(second, forward, comparator));
        } else {
            sub_sort(&mut x[..mid_point], forward, comparator);
            sub_sort(&mut x[mid_point..], forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where F: Fn(&T, &T) -> Ordering
{
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::{sort, sort_by};
    use super::{sort, sort_by};
    use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};
    use crate::lib::SortOrder::*;

    #[test]
    fn sort_u32_large() {
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(Ok(()), sort(&mut x, &Ascending));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(Ok(()), sort(&mut x, &Descending));
            assert!(is_sorted_descending(&x));
        }
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro = Student::new("Taro", "yamada", 16);
        let han = Student::new("Han", "Yamada", 14);
        let kyo = Student::new("Kyo", "Ito", 15);
        let ryo = Student::new("Ryo", "Hay", 17);

        let mut x = vec![&taro, &han, &kyo, &ryo];
        let expected = vec![&han, &kyo, &taro, &ryo];
        assert_eq!(
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );
        assert_eq!(expected, x);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "yamada", 10);
        let han = Student::new("Han", "Yamada", 14);
        let kyo = Student::new("Kyo", "Ito", 15);
        let ryo = Student::new("Ryo", "Hay", 17);

        let mut x = vec![&taro, &han, &kyo, &ryo];
        let expected = vec![&ryo, &kyo, &han, &taro];
        assert_eq!(
            sort_by(&mut x, &|a, b| a.last_name.cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );
        assert_eq!(expected, x);
    }

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(Ok(()), sort(&mut x, &Ascending));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(Ok(()), sort(&mut x, &Descending));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(Ok(()), sort(&mut x, &Ascending));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(Ok(()), sort(&mut x, &Descending));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string() ,
                age,
            }
        }
    }
}