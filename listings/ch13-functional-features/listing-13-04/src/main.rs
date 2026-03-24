fn main() {
    let liste = vec![1, 2, 3];
    println!("Before defining closure: {liste:?}");

    let sadece_odunc_alir = || println!("From closure: {liste:?}");

    println!("Before calling closure: {liste:?}");
    sadece_odunc_alir();
    println!("After calling closure: {liste:?}");
}
