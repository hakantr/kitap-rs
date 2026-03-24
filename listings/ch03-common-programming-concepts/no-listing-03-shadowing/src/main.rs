fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("İç kapsamdaki x'in değeri: {x}");
    }

    println!("x'in değeri: {x}");
}
