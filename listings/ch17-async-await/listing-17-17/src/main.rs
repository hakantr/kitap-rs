extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: yields
        let a = async {
            println!("'a' başladı.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' bitti.");
        };

        let b = async {
            println!("'b' başladı.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' bitti.");
        };
        // ANCHOR_END: yields

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' {ms}ms çalıştı");
}
