use gunce::Gonderi;

fn main() {
    let mut post = Gonderi::yeni();

    post.metin_ekle("Bugun ogle yemeginde salata yedim");

    let post = post.inceleme_iste();

    let post = post.onayla();

    assert_eq!("Bugun ogle yemeginde salata yedim", post.icerik());
}
