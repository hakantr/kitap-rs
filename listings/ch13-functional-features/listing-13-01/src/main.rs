#[derive(Debug, PartialEq, Copy, Clone)]
enum GomlekRengi {
    Kirmizi,
    Mavi,
}

struct Envanter {
    gomlekler: Vec<GomlekRengi>,
}

impl Envanter {
    fn hediye_et(&self, kullanici_tercihi: Option<GomlekRengi>) -> GomlekRengi {
        kullanici_tercihi.unwrap_or_else(|| self.en_cok_stoklanan())
    }

    fn en_cok_stoklanan(&self) -> GomlekRengi {
        let mut kirmizi_sayisi = 0;
        let mut mavi_sayisi = 0;

        for renk in &self.gomlekler {
            match renk {
                GomlekRengi::Kirmizi => kirmizi_sayisi += 1,
                GomlekRengi::Mavi => mavi_sayisi += 1,
            }
        }
        if kirmizi_sayisi > mavi_sayisi {
            GomlekRengi::Kirmizi
        } else {
            GomlekRengi::Mavi
        }
    }
}

fn main() {
    let magaza = Envanter {
        gomlekler: vec![GomlekRengi::Mavi, GomlekRengi::Kirmizi, GomlekRengi::Mavi],
    };

    let kullanici_tercihi1 = Some(GomlekRengi::Kirmizi);
    let hediye1 = magaza.hediye_et(kullanici_tercihi1);
    println!(
        "{:?} tercihine sahip kullanıcı {:?} alıyor",
        kullanici_tercihi1, hediye1
    );

    let kullanici_tercihi2 = None;
    let hediye2 = magaza.hediye_et(kullanici_tercihi2);
    println!(
        "{:?} tercihine sahip kullanıcı {:?} alıyor",
        kullanici_tercihi2, hediye2
    );
}
