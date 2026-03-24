extern crate trpl; // required for mdbook test

// ANCHOR: here
use std::pin::{Pin, pin};

// --snip--

// ANCHOR_END: here
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (gonderici, mut alici) = trpl::channel();

        let gonderici1 = gonderici.clone();
        // ANCHOR: here
        let gonderici1_gelecegi = pin!(async move {
            // --snip--
            // ANCHOR_END: here
            let degerler = vec![
                String::from("merhaba"),
                String::from("-den"),
                String::from("gelecek"),
                String::from("icinden"),
            ];

            for deger in degerler {
                gonderici1.send(deger).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        // ANCHOR_END: here
        // ANCHOR: here
        let alici_gelecegi = pin!(async {
            // --snip--
            // ANCHOR_END: here
            while let Some(value) = alici.recv().await {
                println!("alındı '{value}'");
            }
            // ANCHOR: here
        });

        let gonderici_gelecegi = pin!(async move {
            // --snip--
            // ANCHOR_END: here
            let degerler = vec![
                String::from("daha"),
                String::from("fazla"),
                String::from("mesaj"),
                String::from("sana"),
            ];

            for deger in degerler {
                gonderici.send(deger).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
            // ANCHOR: here
        });

        let gelecekler: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![gonderici1_gelecegi, alici_gelecegi, gonderici_gelecegi];
        // ANCHOR_END: here

        trpl::join_all(gelecekler).await;
    });
}
