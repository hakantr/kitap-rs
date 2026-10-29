extern crate trpl; // required for mdbook test

// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let degerler = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // --snip--
        // ANCHOR_END: all
        let iter = degerler.iter().map(|n| n * 2);
        let mut akis = trpl::stream_from_iter(iter);

        while let Some(deger) = akis.next().await {
            println!("Değer şuydu: {deger}");
        }
    });
}
