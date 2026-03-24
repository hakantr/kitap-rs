fn main() {
    let mut liste = vec![1, 2, 3];
    println!("Before defining closure: {liste:?}");

    let mut degistirilebilir_odunc_alir = || liste.push(7);

    degistirilebilir_odunc_alir();
    println!("After calling closure: {liste:?}");
}
