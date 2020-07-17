fn main() {
    f4_2();
    f4_3();
}

fn f4_3() {
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    let mut t2 = (88, true);
    t2.1 = false;
    // t2.1 = 1; // error
    assert_eq!(t2.1, false);

    let (n1, b1) = t2;
    println!("({}, {})", n1, b1);
    let ((_, _), (n2, _)) = ((1, 2), (3, 4));
    println!("{}", n2);
    let mut t1 = ((1, 2), (3, 4));
    let ((ref mut x1_ptr, ref mut x2_ptr), _) = t1;
    *x1_ptr = 5;
    *x2_ptr = 6;
    println!("{:?}", t1);

    let a1 = [false, true, false];
    let a2 = [0.0, 1.0, 0.6, -0.4];
    assert_eq!(a1.len() + 1, a2.len());
    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);
    let _a4 = [['a', 'b'], ['c', 'd']];
    let i = 99;
    let mut v = vec![0; i];
    assert_eq!(v.len(), 99);
    v.push(1);
    assert_eq!(v.len(), 100);
    assert_eq!(v.pop().unwrap(), 1);
    assert_eq!(v.len(), 99);
    let mut a1 = ['h', 'e', 'l', 'l', 'o'];
    println!("{}", a1[1]);
    a1[0] = 'H';
    println!("{}", a1[0]);
    let i = 0;
    println!("{}", a1[i]);
    println!("{:?}", a1.get(4));
    println!("{}", a1.get(4).unwrap());
    println!("{:?}", a1.get(5));
    
    let mut a4 = [1; 50];
    for i in a4.iter() {
        print!("{}", i);
    }
    println!();
    for i in a4.iter_mut() {
        print!("{}", i);
    }
    println!();

    fn print_info(name: &str, sl: &[char]) {
        println!("  {:9} - {},  {}, {:?}, {:?}, {:?}",
            name,
            name,
            sl.len(),
            sl.first(),
            sl[1],
            sl.last(),
        );
    }

    let a1 = ['a', 'b', 'c', 'd', 'e'];
    println!("a1: {:?}", a1 );
    print_info("&a1[..]", &a1);
    print_info("&a1[..]", &a1[..]);
    let v1 = vec!['f', 'g', 'h', 'i', 'j'];
    print_info("&v1[..]", &v1);
    
    let mut a1 = [5, 4, 3, 2];
    let s1 = &mut a1[1..3];
    s1[0] = 6;
    s1[1] *= 10;
    s1.swap(0, 1);
    println!("{:?}", s1);
    println!("{:?}", a1);

    let a2: [i32; 0] = [];
    let s2 = &a2;
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.get(0), None);

    let a3 = ["Zero", "One", "Two", "Three", "Four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"One"));
    assert_eq!(s3[1], "Two");
    assert_eq!(s3.get(1), Some(&"Two"));
    assert_eq!(a3.get(1), Some(&"One"));
    assert!(a3.contains(&"One"));
    assert!(s3.starts_with(&["One", "Two"]));
    assert!(s3.ends_with(&["Two", "Three"]));

    let mut a4 = [1, 3, 4, 5];
    println!("{:?}", a4);
    assert_eq!(a4.get(1).unwrap(), &3);
    let i1 = a4.get_mut(3).unwrap();
    println!("{:?}", i1);
    *i1 = 10;
    println!("{:?}", i1);
    println!("{:?}", a4);
    let mut i2 = a4.get_mut(3).unwrap();
    println!("{:?}", i2);
    i2 = a4.get_mut(1).unwrap();
    println!("{:?}, warning is not displayed", i2);
    let _i3 = a4.get(1).unwrap();
    // *i3 = 1; // error

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];
    a4[2..6].sort();
    println!("{:?}", a4);
    let (s4a, s4b) = a4.split_at_mut(5);
    s4a.reverse();
    println!("s4a: {:?}", s4a);
    s4b.sort_unstable();
    println!("s4b: {:?}", s4b);
    println!("{:?}", a4);

    let s1 = "abc1";
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);
    let s3 = "add new
              line";
    let s4 = "no new \
              line";
    println!("{}", s3);
    println!("{}", s4);
    let s5 = "\\ add back slash by escape";
    let s6 = r#"\ add back slash by row string"#;
    println!("{}", s5);
    println!("{}", s6);
    let s7 = r###"it's able to use # and ## in this literal"###;
    println!("{}", s7);
    println!("{}\u{1f600}",s7 );

    let fruits = "red apple, green apple\nraspberry, black berry";
    let mut lines = fruits.lines();
    let apple_line = lines.next();
    assert_eq!(Some("red apple, green apple"), apple_line);
    assert_eq!(Some("raspberry, black berry"), lines.next());
    assert_eq!(None, lines.next());

    if let Some(apples) = apple_line {
        assert!(apples.starts_with("red"));
        assert!(apples.contains("apple"));
        println!("{:?}", apples.find("green"));
        // let mut apple_iter = apples.split(", "); //able to use split by some strings
        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("red apple"));
        let green = apple_iter.next();
        assert_eq!(Some(" green apple"), green);
        assert_eq!(Some("green apple"), green.map(|s| s.trim()));
        assert_eq!(Some("green apple"), green.map(str::trim));
    }

    let s1 = "a";
    let s2 = "あ";
    let s3 = "😀";
    println!("{}, {}, {}", s1.len(), s2.len(), s3.len());
    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..5), None);
    assert_eq!(s.get(3..=6), None );

    let s = "かか\u{3099}く";
    println!("{}", s);
    let s_iter = s.chars();
    for c in s_iter {
        println!("{}", c);
    }
    let mut s_iter = s.chars();
    println!("{:?}", s_iter.next());
    println!("{:?}", s_iter.next());
    println!("{:?}", s_iter.next());
    println!("{:?}", s_iter.next());
    println!("{:?}", s_iter.next());
    println!("{:?}", s_iter.next());

    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));
    let bad_utf8: [u8; 2] = [0x81, 0x33];
    let result = std::str::from_utf8(&bad_utf8);
    assert!(result.is_err());

    let mut s1 = "abc".to_string();
    let s2 = s1.as_mut_str();
    s2.make_ascii_uppercase();
    assert_eq!("ABC", s2);

    let b = unsafe {s2.as_bytes_mut() };
    b[0] = b'D';
    b[1] = b'E';
    b[2] = b'F';
    // println!("{}", s1);
    println!("{}", s2);
}

fn f4_2() {
    let n = 42;
    let c = 'R';
    println!("{}, {}", n, c);

    fn hello() {
        println!("hello");
    }

    let hello = hello();
    assert_eq!((), hello);
    assert_eq!(std::mem::size_of::<()>(), 0);

    let b1 = true;
    let b2 = !b1;
    assert_eq!(b2, false);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 > 10;
    let b4 = n2 > 10;
    let b5 = b3 && b4;
    let b6 = b3 || b4;
    assert_eq!(b5, false);
    assert_eq!(b6, true);
    assert_eq!(std::mem::size_of::<bool>(), 1);

    let n1 = 10_000;
    let n1_1 = 10000;
    let n2 = 0u8;
    let n2_1: u8 = 0;
    let n3 = -100_isize;
    let n3_1: isize = -100;
    assert_eq!(n1, n1_1);
    assert_eq!(n2, n2_1);
    assert_eq!(n3, n3_1);

    let n4 = 10;
    let n5 = n3 + n4;
    print_typename(n5);

    let n1 = 57;
    let h1 = 0xff;
    let o1 = 0o71;
    let b1 = 0b1111_1111;
    assert_eq!(h1, b1);
    assert_eq!(n1, o1);

    let n6 = b'A';
    assert_eq!(n6, 65u8);
    
    let n1 = std::u8::MAX;
    let _n2 = 1u8;
    println!("{}", n1);
    // n1 + n2 >> overflow only debug mode
    let n1 = 200u8;
    let n2 = 3u8;
    assert!(n1.checked_mul(n2).is_none());
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    assert_eq!(n1.wrapping_mul(n2), 88);
    assert_eq!(n1.overflowing_mul(n2), (88, true));

    let _f1 = 10.0;
    let _f2 = -1_234.56_f32;
    let _f3 = 578.6E77;

    let c1 ='A';
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
    let c9 = '\u{1f600}';
    println!("{}, {}, {}, {}, {}, {}, {}", c3, c4, c5, c6, c7, c8, c9);

    fn func1(mut n: u32) {
        println!("func1-1    {}", n);
        n = 1;
        println!("func1-2  {}", n);

    }

    fn func2(n_ptr: &mut u32) {
        println!("func2  *n_ptr  {}", *n_ptr);
        println!("func2  n_ptr  {}", n_ptr);
        *n_ptr = 10;
        println!("func2  *n_ptr  {}", n_ptr);
    }

    let mut x = 5;
    func1(x);
    func2(&mut x);

    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    assert_eq!(*n1_ptr, 0);

    fn double (n: i32) -> i32 {
        n + n
    }

    fn abs(n: i32) -> i32 {
        if n >= 0 { n } else { -n }
    }

    let mut func: fn(i32) -> i32 = double;
    let x = -5;
    println!("{}", func(x));
    func = abs;
    println!("{}", func(x));

    let x = 4;
    let adder = |n| n + x ;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };
    println!("{}", flipflop());
    println!("{}", flipflop());
    println!("{}", flipflop());
    println!("{}", state);
}

fn print_typename<T>(_ : T) {
    println!("{}", std::any::type_name::<T>());
}