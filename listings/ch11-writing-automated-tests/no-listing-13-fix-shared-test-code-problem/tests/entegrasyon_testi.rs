use toplayici::iki_ekle;

mod ortak;

#[test]
fn iki_ekliyor() {
    ortak::kurulum();

    let sonuc = iki_ekle(2);
    assert_eq!(sonuc, 4);
}
