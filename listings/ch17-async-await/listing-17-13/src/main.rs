extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
        let (gonderici, mut alici) = trpl::channel();

        let gonderici1 = gonderici.clone();
        let gonderici1_gelecegi = async move {
            let degerler = vec![
                String::from("merhaba"),
                String::from("gelecekten"),
                String::from("selam"),
                String::from("var"),
            ];

            for deger in degerler {
                gonderici1.send(deger).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let alici_gelecegi = async {
            while let Some(alinan) = alici.recv().await {
                println!("alındı '{alinan}'");
            }
        };

        let gonderici_gelecegi = async move {
            let degerler = vec![
                String::from("daha"),
                String::from("fazla"),
                String::from("mesaj"),
                String::from("sana"),
            ];

            for deger in degerler {
                gonderici.send(deger).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(gonderici1_gelecegi, gonderici_gelecegi, alici_gelecegi);
        // ANCHOR_END: here
    });
}
