#[tokio::main]
async fn main() {
    let result = app::start().await;

    if let Err(err) = result {
        eprintln!("{err:?}")
    }
}
