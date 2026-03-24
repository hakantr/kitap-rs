fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut skorlar = HashMap::new();
    skorlar.insert(String::from("Mavi"), 10);

    skorlar.entry(String::from("Sarı")).or_insert(50);
    skorlar.entry(String::from("Mavi")).or_insert(50);

    println!("{skorlar:?}");
    // ANCHOR_END: here
}
