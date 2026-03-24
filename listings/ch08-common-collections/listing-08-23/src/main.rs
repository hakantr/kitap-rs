fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut skorlar = HashMap::new();

    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Mavi"), 25);

    println!("{skorlar:?}");
    // ANCHOR_END: here
}
