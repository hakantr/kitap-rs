// ANCHOR: here
enum MadeniPara {
    Kurus,
    BesKurus,
    OnKurus,
    YirmiBesKurus,
}

fn kurus_degeri(para: MadeniPara) -> u8 {
    match para {
        MadeniPara::Kurus => 1,
        MadeniPara::BesKurus => 5,
        MadeniPara::OnKurus => 10,
        MadeniPara::YirmiBesKurus => 25,
    }
}
// ANCHOR_END: here

fn main() {}
