fn main() {
    // ANCHOR: here
    let vektor = vec!['a', 'b', 'c'];

    for (indeks, deger) in vektor.iter().enumerate() {
        println!("{deger}, {indeks}. indekste");
    }
    // ANCHOR_END: here
}
