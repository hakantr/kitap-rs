extern crate trpl; // required for mdbook test

fn main() {
    trpl::block_on(async {
        // ANCHOR: stream
        let degerler = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let yineleyici = degerler.iter().map(|n| n * 2);
        let mut akis = trpl::stream_from_iter(yineleyici);

        while let Some(value) = akis.next().await {
            println!("Değer şuydu: {value}");
        }
        // ANCHOR_END: stream
    });
}
