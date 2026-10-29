extern crate trpl; // required for mdbook test

fn main() {
    trpl::block_on(async {
        // ANCHOR: stream
        let degerler = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = degerler.iter().map(|n| n * 2);
        let mut akis = trpl::stream_from_iter(iter);

        while let Some(deger) = akis.next().await {
            println!("Değer şuydu: {deger}");
        }
        // ANCHOR_END: stream
    });
}
