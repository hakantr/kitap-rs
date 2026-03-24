extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let bir_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' başladı.");
            slow("a", 30);
            trpl::sleep(bir_ms).await;
            slow("a", 10);
            trpl::sleep(bir_ms).await;
            slow("a", 20);
            trpl::sleep(bir_ms).await;
            println!("'a' bitti.");
        };

        let b = async {
            println!("'b' başladı.");
            slow("b", 75);
            trpl::sleep(bir_ms).await;
            slow("b", 10);
            trpl::sleep(bir_ms).await;
            slow("b", 15);
            trpl::sleep(bir_ms).await;
            slow("b", 350);
            trpl::sleep(bir_ms).await;
            println!("'b' bitti.");
        };
        // ANCHOR_END: here

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' {ms}ms çalıştı");
}
