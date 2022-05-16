use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let feeds = (0..10).collect::<Vec<_>>();
    let res = read_feeds(feeds).await;
    dbg!(res);
    dbg!(start.elapsed());

    Ok(())
}

async fn read_feeds(feeds: Vec<u32>) -> Vec<u32> {
    feeds
        .iter()
        .map(read_feed)
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
}

async fn read_feed(feed: &u32) -> u32 {
    sleep(Duration::from_millis(200)).await;

    feed * 2
}
