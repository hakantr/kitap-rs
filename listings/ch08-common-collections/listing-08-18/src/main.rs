fn main() {
    // ANCHOR: here
    let s1 = String::from("Merhaba, ");
    let s2 = String::from("dünya!");
    let s3 = s1 + &s2; // not: s1 buraya taşındı ve artık kullanılamaz
                       // ANCHOR_END: here
}
