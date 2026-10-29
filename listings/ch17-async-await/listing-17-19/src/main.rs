extern crate trpl; // required for mdbook test

use std::time::Duration;

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

// ANCHOR: declaration
async fn timeout<F: Future>(
    denenecek_future: F,
    azami_sure: Duration,
) -> Result<F::Output, Duration> {
    // Uygulamamız buraya gelecek!
}
// ANCHOR_END: declaration
