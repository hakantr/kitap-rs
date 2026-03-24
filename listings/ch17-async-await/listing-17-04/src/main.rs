extern crate trpl; // required for mdbook test

use trpl::Html;

// ANCHOR: run
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match sayfa_basligi(url).await {
            Some(title) => println!("{url} için başlık {title} idi"),
            None => println!("{url} için başlık yoktu"),
        }
    })
}
// ANCHOR_END: run

async fn sayfa_basligi(url: &str) -> Option<String> {
    let yanit_metni = trpl::get(url).await.text().await;
    Html::parse(&yanit_metni)
        .select_first("title")
        .map(|title| title.inner_html())
}
