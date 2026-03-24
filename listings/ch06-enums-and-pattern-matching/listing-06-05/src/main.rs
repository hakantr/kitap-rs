fn main() {
    // ANCHOR: here
    fn bir_ekle(x: Option<i32>) -> Option<i32> {
        match x {
            // ANCHOR: first_arm
            None => None,
            // ANCHOR_END: first_arm
            // ANCHOR: second_arm
            Some(i) => Some(i + 1),
            // ANCHOR_END: second_arm
        }
    }

    let bes = Some(5);
    let alti = bir_ekle(bes);
    let hicbiri = bir_ekle(None);
    // ANCHOR_END: here
}
