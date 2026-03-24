use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let karsilama_dosyasi = File::open("merhaba.txt")?;

    Ok(())
}
