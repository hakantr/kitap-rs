fn main() {
    // ANCHOR: here
    let mut metin = String::from("merhaba");

    metin.push_str(", dünya!"); // push_str() bir String'e sabit metin ekler

    println!("{metin}"); // bu `merhaba, dünya!` yazdıracak
                         // ANCHOR_END: here
}
