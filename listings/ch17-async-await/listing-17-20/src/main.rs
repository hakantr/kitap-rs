extern crate trpl; // required for mdbook test

use std::time::Duration;

// ANCHOR: implementation
use trpl::Either;

// --snip--

// ANCHOR_END: implementation
fn main() {
    trpl::block_on(async {
        let yavas = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Sonunda bitti"
        };

        match timeout(yavas, Duration::from_secs(2)).await {
            Ok(message) => println!("'{message}' ile başarılı oldu"),
            Err(duration) => {
                println!("{} saniye sonra başarısız oldu", duration.as_secs())
            }
        }
    });
}

// ANCHOR: implementation
async fn timeout<F: Future>(
    denecek_gelecek: F,
    azami_sure: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(denecek_gelecek, trpl::sleep(azami_sure)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(azami_sure),
    }
}
// ANCHOR_END: implementation
