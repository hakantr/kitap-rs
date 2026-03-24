unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("C'ye gore -3'un mutlak degeri: {}", abs(-3));
}
