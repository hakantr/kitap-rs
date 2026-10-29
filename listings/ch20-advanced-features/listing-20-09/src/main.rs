unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("C'ye göre -3'ün mutlak değeri: {}", abs(-3));
}
