extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: many-messages
        let (gonderici, mut alici) = trpl::channel();

        let degerler = vec![
            String::from("merhaba"),
            String::from("gelecekten"),
            String::from("selam"),
            String::from("var"),
        ];

        for deger in degerler {
            gonderici.send(deger).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(alinan) = alici.recv().await {
            println!("alındı '{alinan}'");
        }
        // ANCHOR_END: many-messages
    });
}
