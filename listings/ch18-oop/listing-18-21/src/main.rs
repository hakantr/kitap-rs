use gunce::Gonderi;

fn main() {
    let mut gonderi = Gonderi::yeni();

    gonderi.metin_ekle("Bugün öğle yemeğinde salata yedim");

    let gonderi = gonderi.inceleme_iste();

    let gonderi = gonderi.onayla();

    assert_eq!("Bugün öğle yemeğinde salata yedim", gonderi.icerik());
}
