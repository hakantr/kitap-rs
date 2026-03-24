extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: slow-futures
        let a = async {
            println!("'a' başladı.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' bitti.");
        };

        let b = async {
            println!("'b' başladı.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' bitti.");
        };

        trpl::select(a, b).await;
        // ANCHOR_END: slow-futures
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' {ms}ms çalıştı");
}
