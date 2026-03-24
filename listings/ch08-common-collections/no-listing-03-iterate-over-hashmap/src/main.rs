fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut skorlar = HashMap::new();

    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Sarı"), 50);

    for (anahtar, deger) in &skorlar {
        println!("{anahtar}: {deger}");
    }
    // ANCHOR_END: here
}
