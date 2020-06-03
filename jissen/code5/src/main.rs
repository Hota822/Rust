// use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let s = "aa";
    print_typename(s);

    // 5-2-1
    let t1 = (3, "birds".to_string());
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));

    // 5-2-2

    let _v1 = vec!(false, true, false);
    let v2 = vec!(0.0, -1.0, 1.0, 0.5);
    assert_eq!(v2.len(), 4);

    let v3 = vec!(0; 100);
    assert_eq!(v3.len(), 100);

    let _v4 = vec!(vec!('a', 'b', 'c'), vec!('d'));
    // let v5 = vec!(false, 'a'); // error mismatch type

    let mut v6 = vec!('a', 'b', 'c');
    // let mut v6 = ['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f');
    assert_eq!(v6, ['a', 'f', 'b', 'c', 'd']);
    assert_eq!(v6.remove(2), 'b');
    assert_eq!(v6, ['a', 'f', 'c', 'd']);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []);

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);

    let v9: Vec<i32> = Vec::new();
    let v10: Vec<i32> = Vec::with_capacity(2);
    assert_eq!(v9, []);
    assert!(v10.is_empty());

    let mut v1 = vec!(0, 1, 2, 3);
    v1.push(4);
    f1(&v1);
    let s1 = v1.into_boxed_slice();
    let v2 = s1.into_vec();
    f1(&v2);

    let mut v1 = vec!(0, 1, 2, 3);
    v1.push(4);
    f1(&v1);
    v1.shrink_to_fit();
    f1(&v1);

    // 5-2-3
    // use std::collections::HashMap;
    let mut m1 = std::collections::HashMap::new();
    // let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);

    m1.insert("d", 2);
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&9));
    // assert_eq!(m1.get("d"), Some(&7));

    // 5-2-4
    let mut s1 = "ラズベリー".to_string();
    let s2 = String::from("ブラックベリー");
    s1.push_str("タルト");
    s1.push('と');
    s1.push_str(&s2);
    println!("{}", s1);
    let i = 42;
    assert_eq!(i.to_string(), "42");
    let fl1 = 4.3 + 0.1;
    assert_eq!(fl1.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", fl1), "4.40");
    let t1 = (1, "ABC");
    assert_eq!(format!("{:?}", t1), r#"(1, "ABC")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));
    let s2 = "ABC";
    let r1: Result<f64, _> = s1.parse();
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r1);
    println!("{:?}", r2);

    let cs = ['t', 'r', 'u', 's', 't'];
    assert_eq!(cs.iter().collect::<String>(), "trust");

    let bad_utf:[u8; 7] = [
        b'a',
        0xf0, 0x90, 0x80,
        0xe3, 0x81, 0x82
    ];
    let s = String::from_utf8_lossy(&bad_utf);
    println!("{}", s);

    // 5-2-5

    let a = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(a[ ..  ], ['a', 'b', 'c' ,'d', 'e']);
    assert_eq!(a[ .. 3], ['a', 'b', 'c'          ]);
    assert_eq!(a[ ..=3], ['a', 'b', 'c' ,'d'     ]);
    assert_eq!(a[1..  ], [     'b', 'c' ,'d', 'e']);
    assert_eq!(a[1.. 3], [     'b', 'c'          ]);

    assert_eq!(  ..    , std::ops::RangeFull);
    assert_eq!(  ..3   , std::ops::RangeTo { end: 3 });
    assert_eq!(  ..=3  , std::ops::RangeToInclusive{ end: 3 });
    assert_eq!( 1..    , std::ops::RangeFrom {start: 1});
    assert_eq!( 1..3   , std::ops::Range {start: 1, end: 3});
    assert_eq!( 1..=3  , std::ops::RangeInclusive::new(1, 3));


    // 5-2-6
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(5), None);

    let mut o1 = Some(10);
    let mut result = 0;
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }
    result += match o1 {
        Some(s) => s,
        None => unreachable!(),
    };
    println!("{}", result);

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    }
    // NG
    // result = if let Some(s) = o1 {
        // s;
    // }

    let mut o2 = Some(String::from("Hello"));
    // let mut o2 = Some("Hello"); // error: String is required
    assert_eq!(o2.unwrap(), "Hello");
    o2 = None;
    // o2.unwrap(); // panic
    assert_eq!(o2.unwrap_or_else(|| "o2 is none".to_string()), "o2 is none");
    o2 = None;
    assert_eq!(o2.unwrap_or_else(|| String::from("o2 is none")), "o2 is none");

    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);
    
    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
          .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );
    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
          .and_then(|n| Some(n * 10)),
        Some(1000)
    );

    let a1 = [3, 7, 31, 127];
    assert_eq!(elements(&a1), Some(3 + 127));
    assert_eq!(bad_elements(&a1), None);

    // 5-2-7
    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("3", "abc").is_err());
    println!("{:?}", add1("aaa", "abd"));

    // let mut r1: Result<i32, std::num::ParseFloatError> = Ok(10);
    let r1: Result<i32, std::num::ParseFloatError> = Ok(10);
    let mut o1 = r1.ok();
    println!("{:?}", o1);
    let r2 = o1.ok_or_else(||"none");
    println!("{:?}", r2);

    o1 = None;
    println!("{:?}", o1);
    let r2 = o1.ok_or_else(||"none");
    println!("{:?}", r2);

    // 5-3-1
    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user("aaa".to_string(), id, now);
    println!("{:?}", user);

    // 5-3-2
    let _s1 = Polygon {
        vertexes: vec!((1, 1)),
        stroke_width: 1,
        fill: (1,1,1),
    };
    let s2 = Vertex(1,1);
    let s3 = Vertex(2,1);
    let s4 = Vertex(3,1);
    let _s5 = Triangle(s2, s3, s4);
    let poly = new_poly(vec!((1,2), (3,4), (5, 6)));
    // assert_eq!(poly.vertexes, [(1,2), (3,4), (5, 6)]);
    assert_eq!(poly.vertexes, vec!((1,2), (3,4), (5, 6)));
    assert_eq!(poly.stroke_width, 1);
    assert_eq!(poly.fill, (1,1,1));
    let Polygon { vertexes: v1 , ..} = poly;
    let Polygon { fill, ..} = poly;
    assert_eq!(v1.len(), 3);
    assert_eq!(fill, (1,1,1));

    let mut poly2 = new_poly(vec!((1,2), (3,4), (5, 6)));
    poly2.vertexes.push((2,4));
    assert_eq!(poly2.vertexes.len(), 4);

    let poly3 = new_poly(vec!((1,2), (3,4), (5, 6)));
    let poly4 = Polygon {
        stroke_width: 10,
        .. poly3
    };
    assert_eq!(poly3.stroke_width, 1);
    assert_eq!(poly4.stroke_width, 10);

    let poly5 = Polygon2 {
        .. Default::default()
    };
    assert_eq!(poly5.stroke_width, 99);
    println!("{:?}", poly5.vertexes);
    println!("{:?}", poly5.fill);

    let s2 = Vertex(1,2);
    let s3 = Vertex(3,4);
    let s4 = Vertex(5,6);
    let s5 = Triangle(s2, s3, s4);
    assert_eq!((s5.0).1, 2);

    let id = Id2(400);
    let now = Timestamp2(4567890123);
    let _bad_user = new_user2(UserName2("bbb".to_string()), id, now);
    // let bad_user = new_user2(UserName2("bbb".to_string()), now, id); // error

    let uni = AAA;
    let uni2 = AAA;
    let _uni3 = UniValue;
    assert_eq!(uni, uni2);
    // assert_eq!(uni, uni3); // error

    // 5-3-3
    say_something(Weekday::Monday);
    say_something(Weekday::Friday);

    assert_eq!(2, Month::February as isize);

    
    use crate::Task::*;
    let tasks = vec![
        AssignedTo(String::from("aaaa")),
        Working {
            assignee: String::from("bbbb"),
            remaining_hours: 18,
        },
        Done
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("Task: {} is assigned to {}", i, assignee)
            },
            Working { assignee, remaining_hours } => {
                println!("{} is working on task: {}. {} hours remaining ", i, assignee, remaining_hours)
            },
            _ => println!("Task: {} is other status({:?})", i, task)
        }
    }

    // 5-3-4
    use shape::Polygon3;

    let _s1 = Polygon3 {
        vertexes: vec![(2, 2)],
        .. Default::default()
    };

    let s2 = StrRefs {
        s1: "aaa",
        s2: "bbb",
    };
    println!("s1: {}, s2: {}",s2.s1, s2.s2);

    let vertexes = vec![
        CartesianCoordinates {x: 0.0, y: 0.0},
        CartesianCoordinates {x: 50.0, y: 0.0},
        CartesianCoordinates {x: 30.0, y: 20.0},
    ];
    let poly = Polygon4 { vertexes };
    assert_eq!(poly.vertexes[0].x, 0.0);
    assert_eq!(poly.vertexes[0].y, 0.0);

    let a: A = Default::default();
    println!("struct A ({} bytes)\n  f0: {:p}\n  f1: {:p}\n  f2: {:p}\n", std::mem::size_of::<A>(), &a.f0, &a.f1, &a.f2);

    // 5-4-1
    let i1 = 42;
    let _fl1 = i1 as f64 / 2.5;
    let c1 = 'a';
    assert_eq!(97, c1 as i32);

    let i2 = 300;
    let u1 = i2 as u8;
    assert_eq!(44, u1);

    // 5-4-2

    let t1 = ('a', 42);
    // let t2 = t1 as (u32, u8) // error
    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    // let v2 = v1 as Vec<u16>; // error
    let _t3 = (t1.0 as u32, t1.1 as i8);
    let _v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();
    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v1, v4);

    // 5-4-3
    let p1 = Box::new(10);
    // let p2 = p1 as *mut i32; // error
    let _p3: *mut i32 = unsafe { std::mem::transmute(p1)} ;
    let fl1 = 5.6789e+3_f32;
    let i1 = fl1 as i32;
    println!("{}", i1);
    let i2: i32 = unsafe { std::mem::transmute(fl1) };
    println!("{}", i2);

    let v1: Vec<u8> = vec!(3, 4, 5);
    let v2 = vec![3u8, 4u8, 5u8];
    assert_eq!(v1, v2);
    assert_eq!(Some(&3u8), v1.first());
    assert_eq!(Some(&3u8), (&v1[..]).first());

    let mut s1 = "Type coercion".to_string();
    let s2 = "is actually easy.".to_string();
    s1.push_str(&s2);

    let p1 = &&&&[1,2,3,4];
    let _p2: &[i32; 4] = p1;

    let p3 = &&[1,2,3,4];
    let p4: &[i32; 4] = p3;
    let _p5: &[i32] = p4;
    // let p6: &[i32] = p3; // error

    let mut b1 = Box::new(0);
    let s1 = String::from("deref");
    let v1 = vec![1, 2, 3];
    f2(&mut b1, &s1, &v1);
    assert_eq!(8, *b1);

    let mut v = vec![0; 10];
    f4(&mut v[..]);
    assert_eq!(10 ,v[0]);

    let a1 = [1,2,3,4];
    assert_eq!(1, f5(&a1));
    assert_eq!(1, f6(Box::new(a1)));

    let mut d: Box<dyn std::fmt::Debug>;

    d = Box::new([1, 2]);
    print_typename(d);
    d = Box::new(Some(1));
    print_typename(d);

}

fn f5(p: &[i32]) -> i32 { p[0]}
fn f6(p: Box<[i32]>) -> i32 { p[0]}

fn f3(slice: &[usize]) -> usize {
    slice.len()
}

fn f4(slice: &mut [usize]) {
    let len = f3(slice);
    slice[0] = len;
}

fn f2(n: &mut usize, str: &str, slice: &[i32]) {
    *n = str.len() + slice.len();
}

// #[repr(C)]
#[derive(Default)]
struct A {f0: u8, f1: u32, f2: u8}

pub struct Polygon4<T> {
    pub vertexes: Vec<T>
}

trait Coordinates {}

#[derive(Default)]
struct CartesianCoordinates {
    x: f64,
    y: f64,
}
impl Coordinates for CartesianCoordinates {}

#[derive(Default)]
struct PolarCoordinates {
    _r: f64,
    _theta: f64,
}
impl Coordinates for PolarCoordinates {}

struct StrRefs<'a> {
    s1: &'a str,
    s2: &'a str,
}

mod shape {
    #[derive(Default)]
    pub struct Polygon3 {
        pub vertexes: Vec<(i32, i32)>,
        pub stroke_width: u8,
        pub fill: (u8, u8, u8),
        // internal_id: String,
    }
}

type UserName3 = String;


#[derive(Debug)]
enum Task {
    _Open,
    AssignedTo(UserName3),
    Working {
        assignee: UserName3,
        remaining_hours: u16,
    },
    Done,
}


enum Month {
    _January = 1, February = 2, _March = 3, _December = 12
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("Thanks good. It is Friday");
    } else {
        println!("It is still {:?}.", weekday)
    }
}

#[derive(Debug, PartialEq)]
enum Weekday {
    Monday, _TuesDay, _Wednesday, _Thursday, Friday,
}

#[derive(Debug, PartialEq)]
struct AAA;
#[derive(Debug, PartialEq)]
struct UniValue;

struct UserName2(String);
struct Id2(u64);
struct Timestamp2(u64);
type User2 = (Id2, UserName2, Timestamp2);
fn new_user2(name: UserName2, id: Id2, created: Timestamp2) -> User2 {
    (id, name, created)
}

// #[derive(Default)]
struct Polygon2 {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

impl Default for Polygon2 {
    fn default() -> Self {
        Self {
            stroke_width: 99,
            vertexes: Default::default(),
            fill: Default::default(),
        }
    }
}

struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

// struct UniqueValue;

fn new_poly(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (1,1,1);
    Polygon {vertexes, stroke_width, fill}
}

fn add1(s0: &str, s1: &str) -> Result<i32, String> {
    let s0 = s0.parse::<i32>().map_err(|_e| "s0 is not integer")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s1 is not integer")?;
    Ok(s0 + s1)
}
fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
    let s0 = s0.parse::<i32>()?;
    let s1 = s1.parse::<i32>()?;
    Ok(s0 + s1)
}
fn elements(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}

fn bad_elements(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(5)?;
    Some(s0 + s3)
}
fn f1<T>(v1:  &Vec<T>) {
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
}


fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}