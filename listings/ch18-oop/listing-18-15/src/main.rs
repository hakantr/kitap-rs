use gunce::Gonderi;

fn main() {
    let mut post = Gonderi::yeni();

    post.metin_ekle("Bugun ogle yemeginde salata yedim");
    assert_eq!("", post.icerik());

    post.inceleme_iste();
    assert_eq!("", post.icerik());

    post.onayla();
    assert_eq!("Bugun ogle yemeginde salata yedim", post.icerik());
}
