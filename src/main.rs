mod args;

use std::error::Error;
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::Instant;

use hyper::{Client, Uri};
use tokio::time::{sleep, Duration};

use args::BenchArgs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // parse commandline arguments
    let bench_args = Arc::new(BenchArgs::new(args::prepare_args()));

    let shared_attempt_count = Arc::new(AtomicUsize::new(0));
    let shared_success_count = Arc::new(AtomicUsize::new(0));
    let shared_error_count = Arc::new(AtomicUsize::new(0));

    let start_time = Instant::now();
    for _i in 0..bench_args.concurrency {
        let thread_shared_client = Client::new();
        let thread_attempted = shared_attempt_count.clone();
        let thread_succeeded = shared_success_count.clone();
        let thread_errored = shared_error_count.clone();
        let options = bench_args.clone();
        tokio::spawn(async move {
            let uri = Uri::from_str(&options.url).expect("Could not parse url");
            while thread_attempted.load(SeqCst) < options.total_requests {
                thread_attempted.fetch_add(1, SeqCst);
                match thread_shared_client.get(uri.clone()).await {
                    Ok(_response) => {
                        // TODO: inspect response for success status code
                        thread_succeeded.fetch_add(1, SeqCst);
                    }
                    Err(_error) => {
                        thread_errored.fetch_add(1, SeqCst);
                    }
                }
            }
        });
    }

    // wait until all requests have finished or errored
    while (shared_success_count.load(SeqCst) + shared_error_count.load(SeqCst))
        < bench_args.total_requests
    {
        // TODO: somehow avoid this from getting completely stuck
        sleep(Duration::from_millis(100)).await;
    }

    let elapsed_time = Instant::now() - start_time;
    let attempt_count = shared_attempt_count.load(SeqCst);
    let success_count = shared_success_count.load(SeqCst);
    let error_count = shared_error_count.load(SeqCst);

    println!(
        "Attempted {} total requests: Succeeded {}, Errored {}",
        attempt_count, success_count, error_count
    );
    println!("Time elapsed: {}ms", elapsed_time.as_millis());
    println!(
        "Success Requests/sec: {:.0}",
        1000 as f64 * (success_count as f64) / (elapsed_time.as_millis() as f64)
    );

    Ok(())
}
