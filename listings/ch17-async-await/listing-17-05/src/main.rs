extern crate trpl; // required for mdbook test

// ANCHOR: all
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let baslik_gelecegi_1 = sayfa_basligi(&args[1]);
        let baslik_gelecegi_2 = sayfa_basligi(&args[2]);

        let (url, olasi_baslik) =
            match trpl::select(baslik_gelecegi_1, baslik_gelecegi_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} ilk döndü");
        match olasi_baslik {
            Some(title) => println!("Sayfa başlığı şuydu: '{title}'"),
            None => println!("Başlığı yoktu."),
        }
    })
}

async fn sayfa_basligi(url: &str) -> (&str, Option<String>) {
    let yanit_metni = trpl::get(url).await.text().await;
    let title = Html::parse(&yanit_metni)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
// ANCHOR_END: all
