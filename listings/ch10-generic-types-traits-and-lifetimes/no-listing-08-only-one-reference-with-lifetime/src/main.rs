fn main() {
    let dizgi1 = String::from("abcd");
    let dizgi2 = "efghijklmnopqrstuvwxyz";

    let sonuc = en_uzun(dizgi1.as_str(), dizgi2);
    println!("En uzun dizgi: {sonuc}");
}

// ANCHOR: here
fn en_uzun<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
