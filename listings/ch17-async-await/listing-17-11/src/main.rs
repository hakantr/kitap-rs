extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (gonderici, mut alici) = trpl::channel();

        // ANCHOR: futures
        let gonderici_gelecegi = async {
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
        };

        let alici_gelecegi = async {
            while let Some(alinan) = alici.recv().await {
                println!("alındı '{alinan}'");
            }
        };

        trpl::join(gonderici_gelecegi, alici_gelecegi).await;
        // ANCHOR_END: futures
    });
}
