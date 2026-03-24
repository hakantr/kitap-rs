extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: with-move
        let (gonderici, mut alici) = trpl::channel();

        let gonderici_gelecegi = async move {
            // --snip--
            // ANCHOR_END: with-move
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
        };

        let alici_gelecegi = async {
            while let Some(value) = alici.recv().await {
                println!("alındı '{value}'");
            }
        };

        trpl::join(gonderici_gelecegi, alici_gelecegi).await;
    });
}
