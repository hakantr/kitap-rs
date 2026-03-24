fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let ucuncu: &i32 = &v[2];
    println!("Üçüncü eleman: {ucuncu}");

    let ucuncu: Option<&i32> = v.get(2);
    match ucuncu {
        Some(ucuncu) => println!("Üçüncü eleman: {ucuncu}"),
        None => println!("Üçüncü bir eleman yok."),
    }
    // ANCHOR_END: here
}
