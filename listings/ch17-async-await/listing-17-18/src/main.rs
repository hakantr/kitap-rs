extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: here
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
        // ANCHOR_END: here
    });
}
