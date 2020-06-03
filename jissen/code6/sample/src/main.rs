fn main() {
    let mut circle = Circle {radius: 10};
    println!("Circle's diameter: {}", circle.diameter());
    let circle2: u32 = circle.diameter();
    println!("Circle's diameter: {}", circle2);
    circle.double_diameter();
    println!("Circle's diameter: {}", circle.diameter());
    let circle = Circle::small_circle();
    println!("Circle's diameter: {}", circle.diameter());
    let mut s = "aaa".to_string();
    s.push_str("aaa");
    // println!();

    let light = Light::Green;
    let action = match light {
        Light::Green => "Go",
        Light::_Yellow => "Proceed with caution",
        Light::_Red => "Stop",
    };
    println!("Green: {}", action);

    let unknown = Some("Apple");

    let string = match unknown {
        Some(something) => String::from("Hi, ") + something,
        None => "nothing".to_string()
    };
    println!("{}", string);

    let ten = 10;
    let ten_reference = &ten;
    match ten_reference {
        number => assert_eq!(&10, number)
    }
    match ten_reference {
        &number => assert_eq!(10, number)
    }

    let number = 50;

    let string = match number {
        1 | 2 | 3 => "One or Two or Three",
        40 ..= 50 => "From 40 to 50",
        _ => "Something else",
    };
    println!("{}", string);

    let string = Some("This is a very long string");
    let message = match string {
        Some(s) if s.len() >= 10 => "Long string",
        Some(_) => "Not really",
        None => "Nothing",
    };
    println!("{}", message);

    let score = Some(100);
    if let Some(100) = score {
        println!("You got full marks");
    } else {
        println!("You have mistakes");
    }

    let mut counter = 10;
    let string;
    string = loop {
        print!("{}", counter);
        if counter == 0 { break "aaaaaa\n" }
        counter -= 1;
        if counter < 5 { continue }
        println!("executing");
    };
    println!("{}", string);

    let mut counter = 0;

    while counter != 10 {
        print!("{} ",counter);
        counter += 1
    }
    println!();

    let value = while counter != 20 {
        print!("{} ",counter);
        counter += 2;
        if counter > 15 { break }
    };
    println!();
    assert_eq!(value, ());

    println!("in while");
    let mut counter = Some(0);
    while let Some(i) = counter {
        if i == 10 {
            counter = None;
        } else {
            print!("{}", i);
            counter = Some(i + 1);
        }
    }
    println!();

    let vector = vec!["Cyan", "Magenta", "Yellow", "Black"];

    let ret = for v in vector.iter() {
        println!("{}", v);
    };
    assert_eq!((), ret);
    for (i, v) in vector.iter().enumerate() {
        println!("{}: {}", i+1, v);
    }
    code_6_10a();
    code_6_10b();
    code_6_11a();
    // code_6_11b(); // function not found
    // code_6_11c();
    // code_6_11d();
    code_6_12();
}

mod server {
    pub fn echo() {
        println!("Server");
    }

    pub(crate) fn echo2() {
        println!("print2");
    }

    // pub(in app::network) fn echo() {
        // println!("print3");
    // }
}

mod client {
    pub fn echo() {
        println!("Client");
    }
}

mod network;
fn code_6_12() {
    // use rand::prelude::*;
    network::ping();
    // use server;
    // use client;
    server::echo();
    client::echo();
    server::echo2();
}

fn code_6_10a() {
    let mut one = 1;
    let plus_one = |x| {x + one};
    // one += 1; //error
    println!("6-10a");
    println!("  10 + 1 = {}", plus_one(10));
    println!("  {}", one);
    one += 1;
    println!("  {}", one);
}

fn code_6_10b() {
    println!("6-10b");
    let mut one = 1;
    let plus_one = move |x| {x + one};
    one += 1;
    println!("  {}",one);
    println!("  10 + 1 = {}",plus_one(10));
    println!("  {}",one);

}

#[cfg(foo)]
fn code6_11c(){
    println!("all");
}

#[cfg(color = "blue")]
fn code6_11d(){
    println!("all");
}


// #[macro_use] mod macros;
#[cfg(unix)]
fn code_6_11a() {
    println!("unix");
}

#[cfg(windows)]
fn code_6_11b() {
    println!("windows");
}


enum Light {
    _Red,
    _Yellow,
    Green,
}

struct Circle {
    radius: u32,
}

impl Circle {
    // fn double(&mut self) -> Circle {
    //     self.radius *= 2;
    //     *self
    // }

    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    fn double_diameter(&mut self) -> u32 {
        self.radius *= 2;
        self.diameter()
    }

    fn small_circle() -> Circle {
        Circle {radius: 1}
    }
}
