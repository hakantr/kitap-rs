fn main() {
    let dizgi1 = String::from("abcd");
    let dizgi2 = "xyz";

    let sonuc = en_uzun(dizgi1.as_str(), dizgi2);
    println!("En uzun dizgi: {sonuc}");
}

// ANCHOR: here
fn en_uzun<'a>(x: &str, y: &str) -> &'a str {
    let sonuc = String::from("gerçekten uzun dizgi");
    sonuc.as_str()
}
// ANCHOR_END: here
