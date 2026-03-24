// ANCHOR: here
fn main() {
    let dizgi1 = String::from("uzun dizgi uzundur");
    let sonuc;
    {
        let dizgi2 = String::from("xyz");
        sonuc = en_uzun(dizgi1.as_str(), dizgi2.as_str());
    }
    println!("En uzun dizgi: {sonuc}");
}
// ANCHOR_END: here

fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
