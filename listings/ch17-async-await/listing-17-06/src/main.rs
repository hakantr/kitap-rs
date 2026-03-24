extern crate trpl; // required for mdbook test

// ANCHOR: all
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("birinci görevden merhaba sayı {i}!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("ikinci görevden merhaba sayı {i}!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
// ANCHOR_END: all
