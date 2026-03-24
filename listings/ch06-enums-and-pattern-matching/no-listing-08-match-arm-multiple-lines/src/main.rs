enum MadeniPara {
    Kurus,
    BesKurus,
    OnKurus,
    YirmiBesKurus,
}

// ANCHOR: here
fn kurus_degeri(para: MadeniPara) -> u8 {
    match para {
        MadeniPara::Kurus => {
            println!("Şanslı kuruş!");
            1
        }
        MadeniPara::BesKurus => 5,
        MadeniPara::OnKurus => 10,
        MadeniPara::YirmiBesKurus => 25,
    }
}
// ANCHOR_END: here

fn main() {}
