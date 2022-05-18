use futures::future::join_all;
use log::info;
use pretty_env_logger;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn slowwly(delay_sec: u64) -> reqwest::Url {
    let url = "http://www.google.com";
    sleep(Duration::new(delay_sec, 0));
    reqwest::Url::parse(&url).unwrap()
}

// Now we want to both fetch some data and do some CPU intensive analysis on it
async fn get_and_analyze(n: u8, data: Arc<RwLock<HashMap<u8, u8>>>) -> Result<(u64, u64)> {
    let response: reqwest::Response = reqwest::get(slowwly(1)).await?;
    info!("Dataset {}", n);

    // we get the body of the request
    let txt = response.text().await?;

    let mut map = data.write().expect("RwLock poisoned");

    // We use HashMap::entry to handle the case where another thread
    // inserted the same key while where were unlocked.

    map.entry(n).or_insert_with(|| n * 2 + 1);
    // Let the loop start us over to try again

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

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    // This is new. We can collect futures in a collection. Nice to know!
    let mut futures = vec![];

    let mut map = Arc::new(RwLock::new(HashMap::new()));

    let mut mymap = map.write().expect("RwLock poisoned");
    mymap.insert(1, 15);
    drop(mymap);
    info!("Start process!!");
    for i in 1..=10 {
        let fut = task::spawn(move || {
            async {
                let data = Arc::clone(&map);

                get_and_analyze(i, data)
            }
            .await;
        });
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
