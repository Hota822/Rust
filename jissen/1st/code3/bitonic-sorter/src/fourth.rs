use crate::models::*;
use SortOrder::{ Ascending as Asc, Descending as Desc };
use std::cmp::Ordering;
use rayon;

const PARALLEL_THRESHOLD: usize = 4096;

pub fn sort<T>(x: &mut [T], order: &SortOrder) -> Result<(), String>
where
    T: Ord + Send
{
        match *order {
            Asc => sort_by(x, &|a, b| a.cmp(b)),
            Desc => sort_by(x, &|a, b| b.cmp(a)),
    }
}

pub fn sort_by<T, F>(x: &mut[T], comparator: &F) -> Result<(), String>
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x i not a power of two. (x.len(): {})", x.len()))
    }
}


fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(|| do_sort(first, true, comparator),
                        || do_sort(second, false, comparator));
        } else {
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator)
    }
}


fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(|| sub_sort(first, forward, comparator),
                        || sub_sort(second, forward, comparator));
        } else {
            sub_sort(first, forward, comparator);
            sub_sort(second, forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering
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
    use super::{ sort, sort_by };

    use crate::utils::{ new_u32_vec, is_sorted_ascending, is_sorted_descending };
    use crate::models::*;
    use SortOrder::{ Ascending as Asc, Descending as Desc };

    fn create_data() -> (Student, Student, Student, Student) {
        (
            Student::new("Taro", "Yamada", 16),
            Student::new("Han", "Yamada", 14),
            Student::new("Kyo", "Ito", 15),
            Student::new("Ryo", "Hay", 17),
        )
    }

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

    #[test]
    fn sort_student_by_age_ascending() {
        let (taro, han, kyo, ryo) = create_data();
        let mut x = vec![&taro, &han, &kyo, &ryo];
        let expected = vec![&han, &kyo, &taro, &ryo];

        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let (taro, han, kyo, ryo) = create_data();
        let mut x = vec![&taro, &han, &kyo, &ryo];
        let expected = vec![&ryo, &kyo, &han, &taro];

        assert_eq!(sort_by(&mut x,
            &|a, b| a.last_name.cmp(&b.last_name)
            .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())
        );
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Asc), Ok(()));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Desc), Ok(()));
            assert!(is_sorted_descending(&x));
        }

    }
}