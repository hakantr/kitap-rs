// ANCHOR: all
use gunce::Gonderi;

// ANCHOR: here
fn main() {
    let mut gonderi = Gonderi::yeni();

    gonderi.metin_ekle("Bugün öğle yemeğinde salata yedim");
    assert_eq!("", gonderi.icerik());
    // ANCHOR_END: here

    gonderi.inceleme_iste();
    assert_eq!("", gonderi.icerik());

    gonderi.onayla();
    assert_eq!("Bugün öğle yemeğinde salata yedim", gonderi.icerik());
    // ANCHOR: here
}
// ANCHOR_END: here
// ANCHOR_END: all
