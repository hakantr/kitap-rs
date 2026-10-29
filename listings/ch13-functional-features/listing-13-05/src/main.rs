fn main() {
    let mut liste = vec![1, 2, 3];
    println!("Kapanışı tanımlamadan önce: {liste:?}");

    let mut degistirilebilir_odunc_alir = || liste.push(7);

    degistirilebilir_odunc_alir();
    println!("Kapanışı çağırdıktan sonra: {liste:?}");
}
