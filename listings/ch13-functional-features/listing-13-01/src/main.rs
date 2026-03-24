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
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.gomlekler {
            match color {
                GomlekRengi::Kirmizi => num_red += 1,
                GomlekRengi::Mavi => num_blue += 1,
            }
        }
        if num_red > num_blue {
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
    let giveaway1 = magaza.hediye_et(kullanici_tercihi1);
    println!(
        "{:?} tercihine sahip kullanıcı {:?} alıyor",
        kullanici_tercihi1, giveaway1
    );

    let kullanici_tercihi2 = None;
    let giveaway2 = magaza.hediye_et(kullanici_tercihi2);
    println!(
        "{:?} tercihine sahip kullanıcı {:?} alıyor",
        kullanici_tercihi2, giveaway2
    );
}
