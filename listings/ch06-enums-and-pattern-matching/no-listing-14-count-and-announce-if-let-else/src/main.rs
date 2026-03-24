#[derive(Debug)]
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

fn main() {
    let para = MadeniPara::Kurus;
    // ANCHOR: here
    let mut sayac = 0;
    if let MadeniPara::YirmiBesKurus(eyalet) = para {
        println!("{eyalet:?} eyaletinden çeyreklik!");
    } else {
        sayac += 1;
    }
    // ANCHOR_END: here
}
