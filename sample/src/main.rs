fn main() {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }
    let f: fn(i32) -> i32 = plus_one;
    let x: i32 = f(5);
    println!("{}", x);
}
