fn main() {
    let s1 = "Hello,  ";
    let s2 = "World!";
    let s3 = s1.to_string() + s2;
    assert_eq!(s3, "Hello,  World!");

    hello();
    let ret = hello();
    assert_eq!(ret, ());

    let b1 = true;
    let b2 = !b1;
    assert_eq!(b2, false);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10;
    let b4 = n2 >= 10;
    let b5 = b4 && b3;
    let b6 = b3 || b4;
    assert_eq!(b3, false);
    assert_eq!(b4, true);
    assert_eq!(b5, false);
    assert_eq!(b6, true);
    assert_eq!(std::mem::size_of::<bool>(), 1);

    let n1 = 10_000;
    print_typename(n1);
    let n2 = 0u8;
    print_typename(n2);
    let n3 = -100_isize;

    let n4 = 10;
    let n7 = n4 + n3;
    print_typename(n4);
    print_typename(n7);

    let h1 = 0xff;
    print_typename(h1);
    let o1 = 0o744;
    print_typename(o1);
    let b7 = 0b1010_01110_1110_1001;
    print_typename(b7);

    let n6 = b'A';
    assert_eq!(n6, 65u8);

    // let n1 = std::u8::MAX;
    // let n2 = 1u8;
    // let n3 = n1 + n2;
    // println!("{}", n3);

    let n1 = 200u8;
    let n2 = 3u8;
    assert_eq!(n1.checked_mul(n2), None);
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    assert_eq!(n1.wrapping_mul(n2), 88);
    assert_eq!(n1.overflowing_mul(n2), (88, true));


    let fl1 = 10.0;
    let fl2 = -1_234.56f32;
    let fl3 = 578.6E+77;
    print_typename(fl1);
    print_typename(fl2);
    print_typename(fl3);

    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let c4 = '\t';
    let c5 = '\n';
    let c6 = '\'';
    let c7 = '\\';
    let c8 = '\x7F';
    let c9 = '漢';
    let c10 = '\u{a5b58}';
    let c11 = '\u{1f600}';
    let vec: Vec<char> = vec!(c4, c5, c6, c7, c8, c9, c10, c11);
    print_vec_type(vec);

    // 4-2-6
    let mut n = 0;
    f1(n);
    f2(&mut n);

    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n = 0;
    let n1_ptr = &mut n;
    assert_eq!(*n1_ptr, 0);
    *n1_ptr = 1000;
    assert_eq!(*n1_ptr, 1000);

    // 4-2-7
    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe {*n1_ptr }, 0);
    unsafe {
        *n1_ptr = 2000;
        assert_eq!(*n1_ptr, 2000);
    }

    // 4-2-8
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(2), 4);
    f = abs;
    assert_eq!(f(-24), 24);
    let f_bad = double;
    assert_eq!(f_bad(4), 8);
    // f_bad = abs; // error

    // column
    let x = 4;
    let adder = |n: i32| n + x;
    assert_eq!(adder(2), 6);

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };

    // assert!(!state); // error on this timing: state was mutably borrowed
    assert!(flipflop());
    assert!(!flipflop());
    assert!(flipflop());

    assert!(state);
    // let mut flipflop = || !state;
    // assert!(flipflop());
    // assert!(flipflop());

    // 4-3-1
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert!(t1.1);

    // let i = 0;
    // let t1a = t1.i; // error

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1, (188, true));

    let ((n1, b1), (n2, b2)) = ((88, true), (99, false));
    assert_eq!(n1, 88);
    assert_eq!(b1, true);
    assert_eq!(n2, 99);
    assert_eq!(b2, false);

    // let ((n1, b1), _ ) = ((88, true), (99, false));

    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut x2_ptr), _) = t1;
    *x1_ptr += 3;
    *x2_ptr *= -1;

    assert_eq!(t1, ((3, -5), (10, -1)));

    // 4-3-2
    let a1 = [false, true, false];
    let _a2 = [0.0, -1.0, 1.0, 0.5];
    assert_eq!(a1.len(), 3);
    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);
    let _a4 = [['A', 'B'], ['C', 'D']];
    // let a5 = [false, 1]; // error

    let size = 100;
    // let a1 = [0; size] // error
    let mut v1 = vec!(0; size);
    assert_eq!(v1.len(), 100);
    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);

    let a1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(a1[1], 'e');

    let mut a2 = [0, 1, 2];
    a2[1] = 3;
    assert_eq!(a2, [0, 3, 2]);

    let mut index = 0;
    assert_eq!(a1[index], 'H');
    index += 1;
    assert_eq!(a1[index], 'e');

    let a4 = [1, 2];
    // a4[2] // compile error
    // let index = 3;
    // a4[index] // run time error
    assert_eq!(a4.get(1), Some(&2));
    assert_eq!(a4.get(3), None);

    let a5 = ['a'; 4];
    for ch in a5.iter() {
        print!("{}", *ch);
    }
    println!("");

    let mut a5 = [1; 10];
    for i in a5.iter_mut() {
        *i *= 2;
    }
    assert_eq!(a5, [2; 10]);

    // 4-3-3
    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);
    print_info("&a1[..]", &a1[..]);
    print_info("&a1", &a1);
    print_info("&a1[1..3]", &a1[1..3]);

    let v1 = vec!('e', 'f', 'g', 'h');
    println!("v1: {:?}", v1);
    print_info("&v1[..]", &v1[..]);
    print_info("&v1", &v1);
    print_info("&v1[1..3]", &v1[1..3]);

    let mut a1 = [5, 4, 3, 2];
    let s1 = &mut a1[1..3];
    s1[0] = 6;
    s1[1] *= 10;
    s1.swap(0, 1);
    assert_eq!(s1, [30, 6]);
    assert_eq!(a1, [5, 30, 6, 2]);

    let a1: [i32; 0] = [];
    let s2 = &a1;
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.first(), None);

    let a3 = ["Zero", "One", "Two", "Three", "Four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"One"));
    assert_eq!(s3[1], "Two");
    assert_eq!(s3.get(4), None);
    assert!(s3.contains(&"Two"));
    assert!(s3.starts_with(&["One", "Two"]));
    assert!(s3.ends_with(&["Two", "Three"]));

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7,  5, 1, 7];
    &mut a4[2..6].sort(); // same as a4[2..6].sort()
    assert_eq!(a4[2..6], [0, 2, 8, 9]);
    let (s4a, s4b) = &mut a4.split_at_mut(5);
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // 4-3-4
    let s1 = "abc1";
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);
    let s3 = "aaaaaaa
              bbbbbbbbbbb";
    let s4 = "aaaaaaaaaa\
              bbbbbbbb";
    println!("{}", s3);
    println!("{}", s4);
    let s5 = "\\n";
    let s6 = r#" raw string \, '""", "#;
    let s7 = r####" raw string## ## ##"## ""#"####;
    let s8 = "\u{1f600}";
    let s9 = "\u{1F1EF}\u{1F1F5}";
    println!("{}", s5);
    println!("{}", s6);
    println!("{}", s7);
    println!("{}", s8);
    println!("{}", s9);

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";
    let mut lines = fruits.lines();
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    assert_eq!(lines.next(), None);

    if let Some(apples) = apple_line {
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        assert_eq!(apples.find("あお"), Some(17));
        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));
        let green = apple_iter.next();
        assert_eq!(green, Some(" あおりんご"));
        assert_eq!(green.map(str::trim), Some("あおりんご"));
        // let mut apple_iter = apples.split(", "); // or use this first time
        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!();
    }

    let s1 = "a";
    let s2 = "あ";
    let s3 = "😀";
    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
    assert_eq!(s3.len(), 4);

    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None);

    let s = "かか\u{3099}く";
    println!("{}", s);
    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    iter.next();
    assert_eq!(iter.next(), Some('く'));

    let mut s1 = "abcdef".to_string();
    let s2 = s1.as_mut_str();
    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCDEF");
    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'A';
    b[4] = b'B';
    b[5] = b'C';
    assert_eq!(s2, "ABCABC");

}

fn print_info(name: &str, sl: &[char]) {
    println!("  {:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),
        sl.first(),
        sl.get(1),
        sl.last()
    );
}

fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}

fn f1(mut n: u32) {
    n += 1;
    println!("f1:    n = {}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2:    n_ptr = {:p}", n_ptr);
    *n_ptr = 2;
    println!("f2:    *n_ptr = {}", *n_ptr)
}

fn hello() {
    println!("Hello");
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn print_vec_type<T>(vec: Vec<T>) {
    for value in vec {
        print_typename(value);
    }
}