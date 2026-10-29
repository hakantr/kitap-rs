extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // Daha sonra burada `slow` fonksiyonunu çağıracağız
    });
}

// ANCHOR: slow
fn slow(isim: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{isim}' {ms}ms çalıştı");
}
// ANCHOR_END: slow
