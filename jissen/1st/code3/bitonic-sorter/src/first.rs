pub fn sort(x: &mut [u32], up: bool) {
    // if len(x) <= 1:
    //     return x
    // else:
    //     mid_point = len(x) // 2
    //     first = sort(x[:mid_point], True)
    //     second = sort(x[:mid_point], False)
    //     x1 =first + second
    //     return _sub_sort(x1, up)

    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }

}

fn sub_sort(x: &mut [u32], up: bool) {
    // if len(x) == 1:
    //     return x
    // else:
    //     _compare_and_swap(x, up)
    //     mid_point = len(x) // 2
    //     first = sub_sort(x[:mid_point], up)
    //     second = sub_sort(x[mid_point:], up)
    //     return first + second

    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up)
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    // mid_point = len(x) // 2
    // for i in range(mid_point):
    //     if (x[i] > x[mid_point + i]) == up:
    //         x[i], x[mid_point + i] = x[mid_point + i], x[i]

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

    #[test]
    fn sort_u32_ascending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, true);
        assert_eq!(x, [4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, [330, 110, 30, 21, 20, 11, 10, 4, ]);
    }
}