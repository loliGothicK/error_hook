use error_hook::{self, hook};

#[hook(e => tracing::error!("{e}"))]
#[tracing::instrument]
async fn test(a: i32, b: i32) -> error_hook::Result<i32> {
    (async { a.checked_mul(b).ok_or(anyhow::anyhow!("overflow")) }).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let ans = test(888888888, 888888888).await?;
    println!("{ans}");

    Ok(())
}
