fn main() {
    struct Nokta {
        x: i32,
        y: i32,
    }

    // ANCHOR: here
    let ((ayak, inc), Nokta { x, y }) = ((3, 10), Nokta { x: 3, y: -10 });
    // ANCHOR_END: here
}
