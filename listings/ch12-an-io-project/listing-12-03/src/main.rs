use std::env;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let sorgu = &argumanlar[1];
    let dosya_yolu = &argumanlar[2];

    println!("Searching for {sorgu}");
    println!("In file {dosya_yolu}");
}
