use futures::future::join_all;
use log::{error, info};
use pretty_env_logger::env_logger;
use std::thread;
use tokio::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = "https://jfrog.com";
    thread::sleep(time::Duration::from_millis(delay_ms as u64));
    reqwest::Url::parse(&url).unwrap()
}

// Now we want to both fetch some data and do some CPU intensive analysis on it
async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    let response: reqwest::Response = reqwest::get(slowwly(1000)).await?;

    info!("Dataset {}", n);

    // we get the body of the request
    let txt = response.text().await?;

    // We send our analysis work to a thread where there is no runtime running
    // so we don't block the runtime by analyzing the data
    let res = task::spawn_blocking(move || analyze(&txt)).await?;
    info!("Processed {}", n);
    Ok(res)
}

// Counting the number of ones and zeros in the bytes we get.
fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    // Let's spend as much time as we can and count them in two passes
    let ones = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
}

async fn app() -> Result<()> {
    // This is new. We can collect futures in a collection. Nice to know!
    let mut futures = vec![];

    for i in 1..=10 {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    // Returning errors using `?` in iterators can be a bit difficult. Using a
    // simple for loop to inspect and work with our results can often be more
    // ergonomic
    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    info!(
        "Ratio of ones/zeros: {:.02}",
        total_ones as f64 / total_zeros as f64
    );
    Ok(())
}

fn main() {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(app()) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error ocurred: {}", e),
    };
}
