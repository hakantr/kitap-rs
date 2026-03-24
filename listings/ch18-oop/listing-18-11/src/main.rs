// ANCHOR: all
use gunce::Gonderi;

// ANCHOR: here
fn main() {
    let mut post = Gonderi::yeni();

    post.metin_ekle("Bugun ogle yemeginde salata yedim");
    assert_eq!("", post.icerik());
    // ANCHOR_END: here

    post.inceleme_iste();
    assert_eq!("", post.icerik());

    post.onayla();
    assert_eq!("Bugun ogle yemeginde salata yedim", post.icerik());
    // ANCHOR: here
}
// ANCHOR_END: here
// ANCHOR_END: all
