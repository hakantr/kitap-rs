fn main() {
    // ANCHOR: here
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for deger in v1_iter {
        println!("Alındı: {deger}");
    }
    // ANCHOR_END: here
}
