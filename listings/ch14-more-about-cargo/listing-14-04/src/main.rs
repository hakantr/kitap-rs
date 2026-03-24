use sanat::turler::BirincilRenk;
use sanat::yardimcilar::karistir;

fn main() {
    let kirmizi = BirincilRenk::Kirmizi;
    let sari = BirincilRenk::Sari;
    karistir(kirmizi, sari);
}
