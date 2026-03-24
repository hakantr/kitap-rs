fn main() {
    let sayilar = (2, 4, 8, 16, 32);

    match sayilar {
        (birinci, .., son) => {
            println!("Bazı sayılar: {birinci}, {son}");
        }
    }
}
