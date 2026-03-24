fn main() {
    // ANCHOR: here
    let sayilar = (2, 4, 8, 16, 32);

    match sayilar {
        (birinci, _, ucuncu, _, besinci) => {
            println!("Bazı sayılar: {birinci}, {ucuncu}, {besinci}");
        }
    }
    // ANCHOR_END: here
}
