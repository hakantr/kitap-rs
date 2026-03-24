#[derive(Debug)] // birazdan eyaleti inceleyebilmek için
enum Eyalet {
    Alabama,
    Alaska,
    // --snip--
}

impl Eyalet {
    fn su_tarihte_var_miydi(&self, yil: u16) -> bool {
        match self {
            Eyalet::Alabama => yil >= 1819,
            Eyalet::Alaska => yil >= 1959,
            // -- snip --
        }
    }
}

enum MadeniPara {
    Kurus,
    BesKurus,
    OnKurus,
    YirmiBesKurus(Eyalet),
}

// ANCHOR: describe
fn eyalet_ceyrekligini_tanimla(para: MadeniPara) -> Option<String> {
    let eyalet = if let MadeniPara::YirmiBesKurus(eyalet) = para {
        eyalet
    } else {
        return None;
    };

    if eyalet.su_tarihte_var_miydi(1900) {
        Some(format!("{eyalet:?} Amerika'ya göre oldukça eski!"))
    } else {
        Some(format!("{eyalet:?} nispeten yeni."))
    }
}
// ANCHOR_END: describe

fn main() {
    if let Some(tanim) =
        eyalet_ceyrekligini_tanimla(MadeniPara::YirmiBesKurus(Eyalet::Alaska))
    {
        println!("{tanim}");
    }
}
