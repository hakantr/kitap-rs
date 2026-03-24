extern crate trpl; // required for mdbook test

fn main() {
    // TODO: we'll add this next!
}

// ANCHOR: all
use trpl::Html;

async fn sayfa_basligi(url: &str) -> Option<String> {
    let yanit = trpl::get(url).await;
    let yanit_metni = yanit.text().await;
    Html::parse(&yanit_metni)
        .select_first("title")
        .map(|title| title.inner_html())
}
// ANCHOR_END: all
