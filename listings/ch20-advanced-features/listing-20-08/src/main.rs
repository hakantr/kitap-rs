unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C'ye gore -3'un mutlak degeri: {}", abs(-3));
    }
}
