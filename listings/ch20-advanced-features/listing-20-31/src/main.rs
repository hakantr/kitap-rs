fn main() {
    // ANCHOR: here
    enum Durum {
        Deger(u32),
        Dur,
    }

    let list_of_statuses: Vec<Durum> = (0u32..20).map(Durum::Deger).collect();
    // ANCHOR_END: here
}
