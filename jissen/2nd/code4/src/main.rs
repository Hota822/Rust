fn main() {
    code4_2();
    code4_3();
}

fn code4_3() {
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    let _i = 0;
    // let t1a = t1.i; // error

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1.0, 188);

    let (n1, b1) = t1;
    assert_eq!(n1, 188);
    assert_eq!(b1, true);

    let ((x1, y1), (_, _)) = ((1, 2), (3, 4));
    assert_eq!((x1, y1), (1, 2));

    let mut t1 = (1, 2);
    let (mut x1, mut y1) = t1;
    println!("{}, {}", x1, y1);
    x1 = 3;
    y1 = 4;
    println!("{}, {}", x1, y1);
    let (ref x1_ptr, ref y1_ptr) = t1;
    println!("{}, {}", x1_ptr, y1_ptr);
    let (ref mut x1_ptr, ref mut y1_ptr) = t1;
    *x1_ptr = 5;
    *y1_ptr = 6;
    println!("{}, {}", x1_ptr, y1_ptr);
    assert_eq!(t1, (5, 6));

    let a1 = [false, true, false];
    let _a2 = [0.0, -1.0, 1.0, 0.5];
    assert_eq!(a1.len(), 3);

    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);

    // let a4 = [1, true] // error
    // let a4 = [[1, 2], [3, 4, 5], [5, 6]]; // error
    let a4 = [[1, 2], [3, 4], [5, 6]];
    assert_eq!(a4.len(), 3);

    let size = 100;
    // let a5 = [0; size] // error
    let mut v1 = vec![0; size];
    typename(&v1);
    assert_eq!(v1.len(), 100);

    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);

    let a1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(a1[1], 'e');
    let mut a2 = [1, 2, 3];
    a2[2] = 10;
    assert_eq!(a2, [1, 2, 10]);

    for i in 0..=2 {
        println!("{}",a2[i] );
    }
    assert_eq!(a2.get(1), Some(&2));
    assert_eq!(a2.get(4), None);

    let a4 = ['a'; 50];
    for ch in a4.iter() {
        print!("{}", *ch);
    }
    println!();

    let mut a5 = [1; 50];
    for i in a5.iter_mut() {
        *i *= 2;
    }
    assert_eq!(a5[0], 2);

    fn print_info(name: &str, sl: &[char]) {
        println!("  {:9} - {}, {:?}, {:?}, {:?}",
            name,
            sl.len(),
            sl.first(),
            sl[1],
            sl.last(),
        );
    }

    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);
    print_info("&a1[..]", &a1[..]);
    print_info("&a1", &a1);
    print_info("&a1[1..3]", &a1[1..3]);
    let v1 = vec!['e', 'f', 'g', 'h'];
    println!("v1: {:?}", v1);
    print_info("&v1[..]", &v1[..]);
    print_info("&v1", &v1);
    print_info("&v1[1..3]", &v1[1..3]);

    let mut a1 = [5, 4, 3, 2];
    let s1 = &mut a1[1..=3];
    s1[0] = 6;
    s1.swap(0, 2);
    println!("{:?}", s1);

    let a2: [i32; 0] = [];
    let s2 = &a2;
    assert_eq!(s2.len(), 0);
    assert!(s2.is_empty());
    assert_eq!(s2.first(), None);
    let a3 = ["Zero", "One", "Two", "Three", "Four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"One"));
    assert_eq!(s3[1], "Two");
    assert_eq!(s3.get(1), Some(&"Two"));
    assert_eq!(s3.get(4), None);
    assert!(s3.contains(&"Two"));
    assert!(s3.starts_with(&["One"]));
    assert!(s3.ends_with(&["Three"]));

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];
    &mut a4[2..6].sort();
    assert_eq!(a4, [6, 4, 0, 2, 8, 9, 4, 3, 7, 5, 1, 7]);
    let (s4a, s4b) = &mut a4.split_at_mut(5);
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);


}

fn code4_2() {
    let _n = 42;
    let _c = 'R';

    fn hello() {
        println!("Hello");
    }

    assert_eq!(hello(), ());
    assert_eq!(std::mem::size_of::<()>(), 0);

    let b1 = true;
    let b2 = !b1;
    assert_eq!(b1, true);
    assert_eq!(b2, false);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10;
    let b4 = n2 >= 10;
    let b5 = b3 && b4;
    let b6 = b3 || b4;

    assert_eq!(std::mem::size_of::<bool>(), 1);
    assert_eq!(b5, false);
    assert_eq!(b6, true);

    let _n1 = 10_000;
    let _n2 = 0u8;
    let n3 = 100_isize;
    let n4 = 10;
    let n5 = n4 + n3;
    typename(n5);

    let _h1 = 0xff;
    let _o1 = 0x744;
    let _b1 = 0b1010_0110_1110_1001;

    let n6 = b'A';
    assert_eq!(n6, 65u8);

    let n1 = std::u8::MAX;
    let n2 = 1u8;
    // let n3 = n1 + n2; // error
    let n3 = n1.checked_add(n2);
    assert_eq!(n3, None);

    let n1 = 200u8;
    let n2 = 3u8;

    assert_eq!(n1.checked_mul(n2), None);
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    assert_eq!(n1.wrapping_mul(n2), 88);
    assert_eq!(n1.overflowing_mul(n2), (88, true));
    assert_eq!(n1.overflowing_mul(1), (200, false));

    let _f1 = 10.0;
    let _f2 = 0.1_234_56_f32;
    let _f3 = 578.6e+77;
    let _f4 = 578.6E+77;

    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let _c4 = '\t';
    let _c5 = '\n';
    let _c6 = '\'';
    let _c7 = '\\';
    let _c8 = '\x7F';
    let _c9 = '漢';
    let _c10 = '\u{5b57}';
    let _c11 = '\u{1f600}';
    assert_eq!(std::mem::size_of::<char>(), 4);

    fn func1(mut n: u32) {
        n += 1;
        println!("f1:   n = {}", n);
    }
    fn func2(n_ptr: &mut u32) {
        println!("f2:   n_ptr = {:p}", n_ptr);
        *n_ptr = 2;
        println!("f2:   *n_ptr = {}", *n_ptr)
    }
    let mut n = 0;
    println!("main: n = {}", n);
    func1(n);
    println!("main: n = {}", n);
    func2(&mut n);
    println!("main: n = {}", n);

    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    *n1_ptr = 10;
    assert_eq!(*n1_ptr, 10);

    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!( unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 2;
        assert_eq!(*n1_ptr, 2);
    }

    fn double(n: i32) -> i32 {
        n + n
    }

    fn abs(n: i32) -> i32 {
        if n < 0 { -n } else { n }
    }
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(2), 4);
    f = abs;
    assert_eq!(f(-1), 1);
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());
    println!("{},", std::mem::size_of_val(&f));
    typename(f);

    let mut x = 4;
    let adder = move |n| n + x;
    x = 5;
    println!("{}", x);
    assert_eq!(adder(2), 6);
    let mut state = false;
    let mut flipflop = move || {
        state = !state;
        state
    };
    assert_eq!(flipflop(), true);
    assert_eq!(flipflop(), false);
    assert_eq!(flipflop(), true);
    assert_eq!(state, false);

    let b = 5;
    let mut _f = |a: i32| a * b;
    // f = |a: i32| a * b; // error

    let f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(3), 9);
    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    assert_eq!(v, [1, 4, 5]);
    let v2 = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(str::len)
        .collect::<Vec<_>>();
    assert_eq!(v, v2);
}

fn typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

