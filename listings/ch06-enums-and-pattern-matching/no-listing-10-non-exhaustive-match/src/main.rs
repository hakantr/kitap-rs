fn main() {
    // ANCHOR: here
    fn bir_ekle(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let bes = Some(5);
    let alti = bir_ekle(bes);
    let hicbiri = bir_ekle(None);
}
