use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};
mod structure;
use structure::Child;
use structure::Child2;
use structure::Child3;
use structure::Parent;
use structure::Parent2;



fn main() {
    code7_3();
    code7_6();
    code7_7();
    code7_9();
    code7_9a();
    code7_10();
    if let Ok(()) = code7_11() {
        println!("end code 7-11");
    };
    if let Ok(()) = code7_11a() {
        println!("end code 7-11a");
    }
    code7_12();
}

fn code7_12() {
    println!();
    println!("start 7-12");
    fn apply_fn<F>(f: &F, ch: char) where F: Fn(char) -> bool {
        assert!(f(ch));
    }

    fn apply_fn_mut<F>(f: &mut F, ch: char) where F: FnMut(char) -> bool {
        assert!(f(ch));
    }

    fn apply_fn_once<F>(f: F, ch: char) where F: FnOnce(char) -> bool {
        assert!(f(ch));
    }
    let s1 = "read-only";
    let mut lookup = |ch| s1.find(ch).is_some();
    apply_fn(&lookup, 'r');
    apply_fn_mut(&mut lookup, 'r');
    apply_fn_once(lookup, 'r');

    let mut s2 = "append".to_string();
    let mut modify = |ch| {
        s2.push(ch);
        true
    };
    // apply_fn(&modify, 'e'); // error
    apply_fn_mut(&mut modify, 'e');
    apply_fn_once(modify, 'e');

    let s3 ="be converted".to_string();
    let consume = |ch| {
        let bytes = s3.into_bytes();
        bytes.contains(&(ch as u8))
    };
    // apply_fn_once(&consume, 'b'); // error
    // apply_fn_once(&mut consume, 'b');
    apply_fn_once(consume, 'b');
}

lazy_static! {
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        let dogs = ["aaa", "bbb"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}

fn code7_11a() -> Result<(), Box<dyn Error>> {
    // after 7-11-2.1
    {
        let dogs = DOGS.read()?;
        assert!(dogs.contains("aaa"));
        assert!(dogs.contains("bbb"));
    }
    DOGS.write()?.insert("ccc");

    std::thread::spawn(||
        DOGS.write().map(|mut ds| ds.insert("ddd")).map_err(stringify)
    ).join().expect("Thread error")?;

    assert!(DOGS.read()?.contains("ccc"));
    assert!(DOGS.read()?.contains("ddd"));
    Ok(())
}

fn stringify(x: impl ToString) -> String { x.to_string() }

fn code7_11() -> Result<(), Box<dyn std::error::Error>> {
    use structure::{A, B};
    use std::cell::RefCell;
    println!();
    println!("start 7-11");
    // this is ok
    // let mut a = A { c: 'c', s: "alex".to_string() };
    // let r = &mut a;
    // usable only one of below
    // a.s.push('a');
    // println!("{}", a.s);
    // r.s.push('r');
    // println!("{}", r.s);
    let a = A { c: 'c', s: "alex".to_string() };
    let _r = &a;
    // r.s.push('r'); // error

    let b = B{ c: 'a', s: RefCell::new("alex".to_string()) };
    let rb = &b;
    rb.s.borrow_mut().push('a');
    {
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");
        // let rbsm = b.s.borrow_mut();
        let rbm = b.s.try_borrow_mut();
        assert!(rbm.is_err());
    }
    let rbm = b.s.try_borrow_mut();
    assert!(rbm.is_ok());

    thread_local!(
        static RABBITS: RefCell<HashSet<&'static str>> = {
            let rb = ["aaa", "bbb"].iter().cloned().collect();
            RefCell::new(rb)
        }
    );
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("aaa"));
        rb.borrow_mut().insert("ccc");
    });

    std::thread::spawn(||
        RABBITS.with(|rb| rb.borrow_mut().insert("ddd"))
    ).join().expect("Thread error");

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ccc"));
        assert!(!rb.borrow().contains("ddd"));
    });

    let dogs: HashSet<_> = ["aaa", "bbb"].iter().cloned().collect();
    let dogs = Arc::new(RwLock::new(dogs));

    {
        let ds = dogs.read().map_err(stringify)?;
        assert!(ds.contains("aaa"));
        assert!(ds.contains("bbb"));
    }
    dogs.write().map_err(stringify)?.insert("ccc");
    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        dogs1.write().map(|mut ds| ds.insert("ddd")).map_err(stringify)
    ).join().expect("Thread Error")?;
    assert!(dogs.read().map_err(stringify)?.contains("ccc"));
    assert!(dogs.read().map_err(stringify)?.contains("ddd"));
    Ok(())
 }

fn code7_10() {
    use std::rc::Rc;
    println!();
    println!("start 7-10");
    let mut rc1 = Rc::new(Child3(1));
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);
        println!("(b) count: {}, rc1: {:?}, rc2: {:?}", Rc::strong_count(&rc1), rc1, rc2);
        if let Some(child) = Rc::get_mut(&mut rc1) {
            child.0 += 1;
            println!("unreachable")
        }
    }
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(e) count: {}, rc1: {:?}, weak: {:?}", Rc::strong_count(&rc1), rc1, weak);
    if let Some(child) = weak.upgrade() {
        println!("(f) count: {}, rc1: {:?}, weak: {:?}", Rc::strong_count(&rc1), rc1, child);
    }
    std::mem::drop(rc1);
    println!("(f) count: 0, weak: {:?}", weak.upgrade());


}

fn code7_9a() {
    let array = [1,2,3,4,5,6];
    // error
    // for i in array {
    //     println!("{}", i);
    // }
    let slice = &array[1..5];
    println!("{:?}", slice);
    for (i, _v) in slice.into_iter().enumerate() {
        println!("{}", i);
    }
    // error
    // for i in slice.enumerate() {
        // println!("{}", i);
    // }

    let vector = vec!(1,2,3,4);
    println!("{:?}", vector);
    for i in vector.iter() {
        println!("{}", i);
    }
    let iter = vector.iter();
    // vector.push(5); // error
    // error
    // for i in vector {
    //     println!("{}", i);
    // }
    for i in iter {
        println!("{}", i);
    }
    for i in vector {
        println!("{}", i);
    }
    // error
    // for i in vector {
    //     println!("{}", i);
    // }


}

fn code7_9() {
    use structure::ToyVec;

    println!();
    println!("start 7-9");
    println!();

    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let mut iter = v.iter();
    // v.push("Canary".to_string());
    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string());
    println!("len: {}", v.len());
    println!("pop: {}", v.pop().unwrap());
    println!("len: {}", v.len());
    println!("get_or: {}", v.get_or(4, &"default".to_string()))


}

fn code7_7() {
    fn f1(p: &Parent2) {
        println!("{:?}", p);
    }

    fn f2(p: &mut Parent2) {
        p.0 *= -1;
    }

    let mut p1 = Parent2(1, Child3(11), Child3(21));
    f1(&p1);
    f2(&mut p1);
    println!("{:?}", p1);
}

fn code7_6() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    let p3 = Child2(1);
    let p4 = p3.clone();
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{:?}", p4);
}

fn code7_3() {
    structure::aaa();
    let p1 = Parent(1, Child(11), Child(12));
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }
    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p3: {:?}", p3);

    let array = [1,2,3,4,];
    let [a, b, c, d] = array;
    for v in array.iter() {
        println!("{}", v);
    }
    println!("{}, {}, {}, {}", a, b, c, d);

    for i in 1..5 {
        println!("{}", i);
    }
}
