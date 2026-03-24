fn main() {
    // ANCHOR: here
    let mut metin = String::from("merhaba");

    let r1 = &mut metin;
    let r2 = &mut metin;

    println!("{r1}, {r2}");
    // ANCHOR_END: here
}
