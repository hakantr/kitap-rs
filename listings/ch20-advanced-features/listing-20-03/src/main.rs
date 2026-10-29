fn main() {
    // ANCHOR: here
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 değeri: {}", *r1);
        println!("r2 değeri: {}", *r2);
    }
    // ANCHOR_END: here
}
