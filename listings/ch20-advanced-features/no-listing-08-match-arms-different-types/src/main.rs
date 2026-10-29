fn main() {
    let tahmin = "3";
    // ANCHOR: here
    let tahmin = match tahmin.trim().parse() {
        Ok(_) => 5,
        Err(_) => "merhaba",
    };
    // ANCHOR_END: here
}
