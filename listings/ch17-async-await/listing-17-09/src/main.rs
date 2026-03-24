extern crate trpl; // required for mdbook test

fn main() {
    trpl::block_on(async {
        // ANCHOR: channel
        let (gonderici, mut alici) = trpl::channel();

        let deger = String::from("merhaba");
        gonderici.send(deger).unwrap();

        let alinan = alici.recv().await.unwrap();
        println!("alındı '{alinan}'");
        // ANCHOR_END: channel
    });
}
