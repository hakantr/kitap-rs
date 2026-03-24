extern crate trpl; // required for mdbook test

use std::time::Duration;

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

// ANCHOR: declaration
async fn timeout<F: Future>(
    denecek_gelecek: F,
    azami_sure: Duration,
) -> Result<F::Output, Duration> {
    // Uygulamamiz buraya gelecek!
}
// ANCHOR_END: declaration
