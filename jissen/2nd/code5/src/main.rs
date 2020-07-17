fn main() {
    fn5_2();
    fn5_3();
    fn5_4();
}

fn fn5_4() {
    let i1 = 42;
    let f1 = i1 as f64 / 2.5;
    println!("{:.5}", f1);
    let c1 = 'a';
    assert_eq!(c1 as i32, 97);

    let i2 = 300;
    let u1 = i2 as u8;
    assert_eq!(44, u1);

    let t1 = ('a', 42);
    let _t2 = (t1.0 as u32, t1,1 as u8);

    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    let v2 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();
    let v3 = v1.into_iter().map(|n| n as u8).collect::<Vec<u8>>();
    let v4: Vec<u8> = From::from("hello");
    println!("{:?}", v2);
    assert_eq!(v4, v3);

}


fn fn5_3() {
    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);
    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user("mik".to_string(), id, now);
    println!("{:?}", user);

    // e.g.
    // use std::cell::RefCell;
    // use std::collections::HashMap;
    // use std::rc::Rc;
    // pub type SharedMap<K, V> = Rc<RefCell<HashMap<K, V>>>;

    struct _UniqueValue;

    let _triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    let vertexes = vec![(0, 0), (3, 0), (2, 2)];
    let fill = (255, 255, 255);
    let stroke_width = 1;
    let triangle = Polygon {vertexes, fill, stroke_width};
    println!("{:?}", triangle);
    println!("{}", stroke_width);

    fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
        let stroke_width = 1;
        let fill = (0, 0, 0);
        Polygon {stroke_width, fill, vertexes}
    }
    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    let Polygon { vertexes: vx, .. } = quadrangle;
    assert_eq!(vx, [(5, 2), (4, 7), (10, 6), (8, 1)]);

    let Polygon { fill, .. } = quadrangle;
    assert_eq!(fill, (0, 0, 0));
    // let Polygon { vertexes, .. } = quadrangle; // moved value

    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    polygon.vertexes[0] = (1, 5);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    let polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    let mut b = Box::new(polygon.vertexes);
    b.shrink_to_fit();
    b[0] = (1, 5);
    b.push((2, 8)); // is ok
    println!("before: {}", b.len());
    for _i in 1..=100 {
        b.push((0, 0));
    }
    println!("after: {}", b.len());

    let polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    let mut b = polygon.vertexes.into_boxed_slice();
    b[0] = (1, 5);
    // b.push((2, 8)); // is error

    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 1), (2, 4)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };
    println!("{:?}", triangle1);

    let triangle2 = Polygon {
        fill: (0, 0, 0),
        .. triangle1
    };
    println!("{:?}", triangle2);
    // println!("{:?}", triangle1); // error moved value

    let polygon: Polygon = Default::default();
    println!("{:?}", polygon);

    let polygon = Polygon {
        stroke_width: 4,
        .. Default::default()
    };
    println!("{:?}", polygon);

    // if impl and derive debug at same time, it occurs error
    let polygon: Polygon2 = Default::default();
    println!("{:?}", polygon);

    let polygon = Polygon2 {
        stroke_width: 4,
        .. Default::default()
    };
    println!("{:?}", polygon);

    let v1 = Vertex(0, 1);
    let v2 = Vertex(2, 3);
    let tri = Triangle(v1, v2, Vertex(4, 5));
    println!("{:?}", tri);
    assert_eq!((tri.0).1, 1);

    struct UserName2(String);
    struct Id2(u64);
    struct Timestamp2(u64);
    type User2 = (Id2, UserName2, Timestamp2);
    fn new_user2(name: UserName2, id: Id2, created: Timestamp2) -> User2 {
        (id, name, created)
    }

    let id = Id2(2);
    let created = Timestamp2(4);
    let name: UserName2 = UserName2("name".to_string());
    let _new_user = new_user2(name, id, created);
    // let _new_user = new_user2(name, created, id); // error

    say_something(Weekday::Monday);
    say_something(Weekday::Tuesday);
    say_something(Weekday::Wednesday);
    say_something(Weekday::Thursday);
    say_something(Weekday::Friday);

    assert_eq!(Month::January as i32, 1);
    use crate::Task::*;
    let tasks = vec![
        Open,
        AssignedTo(String::from("jun")),
        Working { assignee: String::from("hir"), remaining_hours: 18 },
        Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(name) => println!("task: {} is assigned to {}", i, name),
            Working { assignee: name, remaining_hours } => {
                println!("task: {} is assigned to {}, now working.{} hours remaining", i, name, remaining_hours)
            },
            Done => (),
            _ => (),
        }
    }

    let vertexes = vec![
        CartesianCoordinate{ x: 0.0, y: 0.0},
        CartesianCoordinate{ x: 50.0, y: 0.0},
        CartesianCoordinate{ x: 30.0, y: 20.0},
    ];

    let poly = Polygon3 { vertexes };
}

#[derive(Default, Debug)]
struct Polygon3<T> {
   pub vertexes: Vec<T>
}

trait Coordinates {}

#[derive(Default)]
struct CartesianCoordinate {
    x: f64,
    y: f64,
}

impl Coordinates for CartesianCoordinate {}


#[derive(Default)]
struct PolarCoordinates {
     r: f64,
     theta: f64,
 }

impl Coordinates for PolarCoordinates {}






struct _StrRefs<'a> {
    s1: &'a str,
    s2: &'a str,
}

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(String),
    Working {
        assignee: String,
        remaining_hours: u16,
    },
    Done,
}

#[derive(Debug, PartialEq)]
enum Month {
    January = 1, _February = 2, _March = 3, _April = 4, _May = 5, _June = 6,
    _July = 7, _August = 8, _September = 9, _October = 10, _November = 11, _December = 12,
}

#[derive(Debug, PartialEq)]
enum Weekday {
    Monday, Tuesday, Wednesday, Thursday, Friday
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("Thanks Good. It's Friday");
    } else {
        println!("It's {:?} yet", weekday);
    }
}

#[derive(Debug)]
struct Triangle(Vertex, Vertex, Vertex);

#[derive(Debug)]
struct Vertex(i32, i32);

#[derive(Debug, Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

#[derive(Debug)]
struct Polygon2 {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}


impl Default for Polygon2 {
    fn default() -> Self {
        Self {
            stroke_width: Default::default(),
            vertexes: Default::default(),
            fill: (255, 255, 255),
        }
    }
}

fn fn5_2() {
    let t1 = (3, "birds".to_string());
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    println!("{:?}", b1);
    assert_eq!(*b1, (4, "birds".to_string()));

    let v1 = vec![false, true, false, false];
    let _v2 = vec![0.0, -1.0, 1.0, 0.5];
    assert_eq!(v1.len(), 4);

    let v3 = [0; 100];
    assert_eq!(v3.len(), 100);

    let _v4 = vec![vec!['a', 'b', 'c',], vec!['d']];
    // let v5 = vec![false, 1]; // error

    let mut v6 = vec!['a', 'b', 'c', ];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);
    assert_ne!(typename(&v6), typename(&['a', 'b', 'c', 'd', 'e']));

    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f');
    v6.remove(2);
    println!("{:?}", v6);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert!(v7.is_empty());
    assert_eq!(v7, []);

    let v8 = vec!['i', 'j'];
    v6.extend_from_slice(&v8);
    println!("{:?}", v6);
    println!("{:?}", v8);

    let mut v1 = vec![0,1, 2, 3];
    v1.push(4);
    println!("len: {}, capacity: {}", v1.len(), v1.capacity());
    let v2 = v1.into_boxed_slice().into_vec();
    println!("len: {}, capacity: {}", v2.len(), v2.capacity());

    use std::collections::HashMap;
    let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    // m1.insert("c", 4);
    assert_eq!(m1.len(), 2);

    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);
    let d = m1.entry("d").or_insert(5);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&12));
    m1.entry("e").or_insert(9);
    println!("{:?}", m1);

    let _h = vec![("f", 2), ("g", 4)].into_iter().collect::<HashMap<_, _>>();

    let mut s1 = "raspberry".to_string();
    let mut s2 = String::from("raspberry");
    let s3 = " tart".to_owned();
    assert_eq!(s1, s2);
    s2 = "strawberry".to_string();
    s1.push_str(" tart");
    for c in " and ".chars() {
        s1.push(c);
    }
    s1.push_str(&s2);
    s1.push_str(&s3);
    println!("{}", s1);

    let i = 42;
    assert_eq!(i.to_string(), "42");
    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.4}", f), "4.4000");
    let t = (1, "abc");
    assert_eq!(format!("{:?}", t), r#"(1, "abc")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>().unwrap(), 42);
    let s2 = "abc";
    let r2: Result<f64 , _> = s2.parse();
    assert!(r2.is_err());

    let cs = ['t', 'r', 's', 'u', 't'];
    println!("{}", cs.iter().collect::<String>());
    println!("{}", cs[1..].iter().collect::<String>());

    let bad_utf8: [u8; 7] = [
        b'a',
        0xf0, 0x90, 0x80,
        0xe3, 0x81, 0x82,
    ];
    let s = String::from_utf8_lossy(&bad_utf8);
    println!("{}", s);

    let a = ['a', 'b', 'c', 'd', 'e'];

    assert_eq!(a[ ..  ], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[ .. 3], ['a', 'b', 'c',         ]);
    assert_eq!(a[ ..=3], ['a', 'b', 'c', 'd',    ]);
    assert_eq!(a[1..  ], [     'b', 'c', 'd', 'e']);
    assert_eq!(a[1.. 3], [     'b', 'c',         ]);
    assert_eq!(a[1..=3], [     'b', 'c', 'd'     ]);

    assert_eq!( ..   , std::ops::RangeFull);
    assert_eq!( .. 3 , std::ops::RangeTo {end: 3});
    assert_eq!( ..=3 , std::ops::RangeToInclusive {end: 3});
    assert_eq!(1..   , std::ops::RangeFrom {start: 1});
    assert_eq!(1.. 3 , std::ops::Range {start: 1, end: 3});
    assert_eq!(1..=3 , std::ops::RangeInclusive::new(1, 3));

    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);

    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    }

    let mut o2 = Some(String::from("Hello"));
    assert_eq!("Hello", o2.unwrap());
    o2 = None;
    let o3 = o2.clone();
    // println!("{}", o2.unwrap()); // panic
    println!("{:?}", o2);
    println!("{}", o2.unwrap_or("World".to_string())); // arg is value
    assert_eq!(o3.unwrap_or_else(|| "o2 is none".to_string()), "o2 is none".to_string()); // arg is closure

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

    fn add_elements(s: &[i32]) -> Option<i32> {
        let s0 = s.get(0)?;
        let s3 = s.get(3)?;
        Some(s0 + s3)
    }

    let s = &[1, 2, 4, 8, 16];
    let n = &[1, 2];

    assert_eq!(add_elements(s).unwrap(), 9);
    assert_eq!(add_elements(n), None);

    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);
    fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
        let s0 = s0.parse::<i32>()?;
        let s1 = s1.parse::<i32>()?;
        Ok(s0 + s1)
    }

    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("a", "b").is_err());

    fn add1(s1: &str, s2: &str) -> Result<i32, String> {
        let s1 = s1.parse::<i32>().map_err(|_e| "s1 is not integer")?;
        let s2: i32 = s2.parse().map_err(|_e| "s2 is not integer")?;
        Ok(s1 + s2)
    }

    assert_eq!(add1("2", "4"), Ok(2 + 4));
    println!("{:?}", add1("a", "2"));
}

fn typename<T>(_: &T) -> &'static str {
    let string = std::any::type_name::<T>();
    println!("{}", string);
    string
}


