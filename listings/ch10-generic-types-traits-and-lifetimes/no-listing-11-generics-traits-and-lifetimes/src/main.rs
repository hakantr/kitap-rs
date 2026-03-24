fn main() {
    let dizgi1 = String::from("abcd");
    let dizgi2 = "xyz";

    let sonuc =
        duyuruyla_en_uzun(dizgi1.as_str(), dizgi2, "Bugün birinin doğum günü!");
    println!("En uzun dizgi: {sonuc}");
}

// ANCHOR: here
use std::fmt::Display;

fn duyuruyla_en_uzun<'a, T>(x: &'a str, y: &'a str, dyr: T) -> &'a str
where
    T: Display,
{
    println!("Duyuru! {dyr}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
