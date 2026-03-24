use std::sync::Mutex;

fn main() {
    let kilit = Mutex::new(5);

    {
        let mut sayi = kilit.lock().unwrap();
        *sayi = 6;
    }

    println!("kilit = {kilit:?}");
}
