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

// ANCHOR: here
fn kurus_degeri(para: MadeniPara) -> u8 {
    match para {
        MadeniPara::Kurus => 1,
        MadeniPara::BesKurus => 5,
        MadeniPara::OnKurus => 10,
        MadeniPara::YirmiBesKurus(eyalet) => {
            println!("{eyalet:?} eyaletinden çeyreklik!");
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    kurus_degeri(MadeniPara::YirmiBesKurus(Eyalet::Alaska));
}
