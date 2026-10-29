extern crate trpl; // for mdbook test

// ANCHOR: all
use std::{thread, time::Duration};

fn main() {
    let (gonderici, mut alici) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            gonderici.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(mesaj) = alici.recv().await {
            println!("{mesaj}");
        }
    });
}
// ANCHOR_END: all
