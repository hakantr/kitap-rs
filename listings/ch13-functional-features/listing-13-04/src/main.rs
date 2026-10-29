fn main() {
    let liste = vec![1, 2, 3];
    println!("Kapanışı tanımlamadan önce: {liste:?}");

    let sadece_odunc_alir = || println!("Kapanıştan: {liste:?}");

    println!("Kapanışı çağırmadan önce: {liste:?}");
    sadece_odunc_alir();
    println!("Kapanışı çağırdıktan sonra: {liste:?}");
}
