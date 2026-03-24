// ANCHOR: here
#[derive(Debug)] // birazdan eyaleti inceleyebilmek için
enum Eyalet {
    Alabama,
    Alaska,
    // --snip--
}

enum MadeniPara {
    Kurus,
    BesKurus,
    OnKurus,
    YirmiBesKurus(Eyalet),
}
// ANCHOR_END: here

fn main() {}
