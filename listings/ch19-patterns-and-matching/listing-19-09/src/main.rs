fn main() {
    let bir_secenek_degeri: Option<i32> = None;
    // ANCHOR: here
    let Some(x) = bir_secenek_degeri else {
        return;
    };
    // ANCHOR_END: here
}
