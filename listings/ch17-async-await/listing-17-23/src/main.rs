extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (gonderici, mut alici) = trpl::channel();

        let gonderici1 = gonderici.clone();
        let gonderici1_gelecegi = async move {
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
        };

        let alici_gelecegi = async {
            while let Some(value) = alici.recv().await {
                println!("alındı '{value}'");
            }
        };

        // ANCHOR: here
        let gonderici_gelecegi = async move {
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
        };

        let gelecekler: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(gonderici1_gelecegi), Box::new(alici_gelecegi), Box::new(gonderici_gelecegi)];

        trpl::join_all(gelecekler).await;
        // ANCHOR_END: here
    });
}
