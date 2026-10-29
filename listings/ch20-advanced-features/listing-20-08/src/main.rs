unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C'ye göre -3'ün mutlak değeri: {}", abs(-3));
    }
}
