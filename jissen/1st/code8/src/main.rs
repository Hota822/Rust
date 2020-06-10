mod models;
use models::*; // relative path
// use crate::models::*; // absolute path
// use models::CartesianCoordinate;
// use models::PolarCoordinate;
// use models::Coordinates;


fn main() {
    code8_1();
    code8_2();
    code8_3();
    code8_4();
    code8_5();

}

fn code8_5() {
    const dim: u32 = CartesianCoordinate::DIMENSION;
    println!("{} = {}", CartesianCoordinate::DIMENSION, dim);


}

fn code8_4() {
    use std::ops::Range;
    use std::iter::Filter;
    use std::fmt;
    fn _to_n(n: i32) -> impl Iterator {
        0..n
    }
    fn _to_n1(n: i32) -> Range<i32> {
        0..n
    }
    fn _to_n_even(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
        (0..n).filter(|i| i % 2 == 0)
    }
    fn _to_n_even1(n:i32) -> impl Iterator {
        (0..n).filter(|i| i % 2 == 0)
    }
    fn one() -> impl fmt::Display {
        1i32
    }
    fn _one1(is_float: bool) -> impl fmt::Display {
        if is_float {
            1.1
        } else {
            // 1 // error
            2.1
        }
    }
    let _n = one();
    // println!("{}", n + n); // error
    fn _gen_counter(init: i32) -> Box<dyn FnMut() -> i32> {
        let mut n = init;
        Box::new( move || {
            let ret = n;
            n += 1;
            ret
        })
    }
    fn _gen_counter1(init: i32) -> impl FnMut() -> i32 {
        let mut n = init;
        move || {
            let ret = n;
            n += 1;
            ret
        }
    }

}

fn code8_3() {
    // use std::string::ToString
    use std::fmt::Display;
    fn stringify<T: ToString>(t: T) -> String {
        t.to_string()
    }
    println!("{}", stringify(2));
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1);
}

fn code8_2() {
    let _data = Box::init("foo");

    let _data = Box::<&str>::init("foo");
    let _data = Box::<f32>::init(0.1);

    let _data: Box<f32> = Init::init(0.1);
    let _data: Box::<f32> = Init::init(0.1);

    let _data: Box<_> = Init::<f32>::init(0.1);

    let _one_u32: u32 = 1.cast();
    let _one_u64: u64 = 1.cast();
}

fn code8_1() {
    let point = (1.0, 1.0);
    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);

    let p = PolarCoordinate::from_cartesian(c);
    println!("r = {}, Θ = {}", p.r, p.theta);

    print_point((0.0, 1.0));
    print_point(PolarCoordinate {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });
    // print_point("ssss"); // error

    let p = (1.0, 0.0).to_cartesian();
    print_point(p.rotate(std::f64::consts::PI));

}


fn print_point<P: Coordinates>(p: P) {
    let p = p.to_cartesian();
    println!("x = {}, y = {}", p.x, p.y);
}

fn _print_point2<P>(p: P)
where
    P: Coordinates,
{
    let p = p.to_cartesian();
    println!("x = {}, y = {}", p.x, p.y);
}

fn _print_point3(p: impl Coordinates) {
    let p = p.to_cartesian();
    println!("x = {}, y = {}", p.x, p.y);
}

fn _print_point4<P>(p: &P)
where
    P: Coordinates + Clone,
{
    let p = p.clone().to_cartesian();
    println!("x = {}, y = {}", p.x, p.y);
}
