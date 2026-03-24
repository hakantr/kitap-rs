fn main() {
    let dizgi1 = String::from("abcd");
    let dizgi2 = "xyz";

    let sonuc = en_uzun(dizgi1.as_str(), dizgi2);
    println!("En uzun dizgi: {sonuc}");
}

// ANCHOR: here
fn en_uzun(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// ANCHOR_END: here
