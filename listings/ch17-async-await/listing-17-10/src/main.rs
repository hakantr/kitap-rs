extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: many-messages
        let (gonderici, mut alici) = trpl::channel();

        let degerler = vec![
            String::from("merhaba"),
            String::from("-den"),
            String::from("gelecek"),
            String::from("icinden"),
        ];

        for deger in degerler {
            gonderici.send(deger).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = alici.recv().await {
            println!("alındı '{value}'");
        }
        // ANCHOR_END: many-messages
    });
}
