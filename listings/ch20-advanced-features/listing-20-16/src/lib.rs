use std::ops::Add;

struct Milimetreler(u32);
struct Metreler(u32);

impl Add<Metreler> for Milimetreler {
    type Output = Milimetreler;

    fn add(self, other: Metreler) -> Milimetreler {
        Milimetreler(self.0 + (other.0 * 1000))
    }
}
