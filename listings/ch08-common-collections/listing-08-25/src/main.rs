fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let metin = "merhaba dünya harika dünya";

    let mut map = HashMap::new();

    for kelime in metin.split_whitespace() {
        let sayac = map.entry(kelime).or_insert(0);
        *sayac += 1;
    }

    println!("{map:?}");
    // ANCHOR_END: here
}
