use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // get user input
    println!("please input wating time");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input number");
    // parse string to num
    let mut input: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if input > 10 {
        println!("too much");
        return;
    }

    // stay time with input value
    // loopp
    loop {
        println!("{} minutes remained", input);
        thread::sleep(Duration::from_millis(1000));
        input -= 1;
        if input == 0 { break; }
    }

    // display string
    let mut i: u8 = 1;
    loop {
        i += 1;
        if i > 100 { break; }
        thread::sleep(Duration::from_millis(100));
        println!("end! {}", i);
    }
}
