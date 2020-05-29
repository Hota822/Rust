fn main() {
    // f1(2);
    // fn3();
    f4_16();
}

fn f1(mut n: u32) {
    n = 1;
    println!("f1    n = {}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2   n_ptr = {:p}", n_ptr);
    *n_ptr = 2;
    // n_ptr = 2 expected &mut, find integer

}
