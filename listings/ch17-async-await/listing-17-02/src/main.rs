extern crate trpl; // required for mdbook test

use trpl::Html;

fn main() {
    // TODO: we'll add this next!
}

async fn sayfa_basligi(url: &str) -> Option<String> {
    // ANCHOR: chaining
    let yanit_metni = trpl::get(url).await.text().await;
    // ANCHOR_END: chaining
    Html::parse(&yanit_metni)
        .select_first("title")
        .map(|title| title.inner_html())
}
