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
    match para {
        MadeniPara::YirmiBesKurus(eyalet) => {
            println!("{eyalet:?} eyaletinden çeyreklik!")
        }
        _ => sayac += 1,
    }
    // ANCHOR_END: here
}
