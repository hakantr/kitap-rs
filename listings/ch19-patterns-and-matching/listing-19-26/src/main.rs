fn main() {
    // ANCHOR: here
    let sayi = Some(4);

    match sayi {
        Some(x) if x % 2 == 0 => println!("{x} sayısı çifttir"),
        Some(x) => println!("{x} sayısı tektir"),
        None => (),
    }
    // ANCHOR_END: here
}
