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
            Ok(mesaj) => println!("'{mesaj}' ile başarılı oldu"),
            Err(sure) => {
                println!("{} saniye sonra başarısız oldu", sure.as_secs())
            }
        }
    });
}

// ANCHOR: implementation
async fn timeout<F: Future>(
    denenecek_future: F,
    azami_sure: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(denenecek_future, trpl::sleep(azami_sure)).await {
        Either::Left(cikti) => Ok(cikti),
        Either::Right(_) => Err(azami_sure),
    }
}
// ANCHOR_END: implementation
