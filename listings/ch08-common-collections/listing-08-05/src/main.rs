fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let olmayan_eleman = &v[100];
    let olmayan_eleman = v.get(100);
    // ANCHOR_END: here
}
