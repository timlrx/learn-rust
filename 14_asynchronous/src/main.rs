mod concurrency;
mod stream;
use std::future::Future;
use trpl::{Either, Html};

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

// Equivalent to the above async function using the async/await syntax
// When Rust sees a function marked with async, it compiles it into a non-async function whose body is an async block
fn page_title2(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

fn run_example() {
    trpl::run(async {
        let url = "http://www.rust-lang.org";
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

fn main() {
    run_example();
    // concurrency::main();
    // concurrency::message_passing();

    stream::main();
    concurrency::race();
    concurrency::distinct_types();
    concurrency::multiple_futures();
}
