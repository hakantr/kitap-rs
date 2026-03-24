fn main() {
    let sayilar = (2, 4, 8, 16, 32);

    match sayilar {
        (.., ikinci, ..) => {
            println!("Bazı sayılar: {ikinci}")
        }
    }
}
