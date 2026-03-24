// ANCHOR: here
fn ilk_satirin_son_karakteri(metin: &str) -> Option<char> {
    metin.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        ilk_satirin_son_karakteri("Merhaba, dünya\nBugün nasılsın?"),
        Some('a')
    );

    assert_eq!(ilk_satirin_son_karakteri(""), None);
    assert_eq!(ilk_satirin_son_karakteri("\nmerhaba"), None);
}
