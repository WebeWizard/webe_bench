mod args;
mod database;

use std::{error::Error, alloc::System};
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::SystemTime;

use hyper::{Client, Uri};
use tokio::{
    sync::watch::error,
    time::{sleep, Duration},
};
use webe_id::WebeIDFactory;

use args::BenchArgs;
use database::BenchResult;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // parse commandline arguments
    let bench_args = Arc::new(BenchArgs::new(args::prepare_args()));

    let shared_attempt_count = Arc::new(AtomicUsize::new(0));
    let shared_success_count = Arc::new(AtomicUsize::new(0));
    let shared_error_count = Arc::new(AtomicUsize::new(0));

    let mut id_factory = WebeIDFactory::new(std::time::UNIX_EPOCH, 0).expect("Failed to build ID factory");
    let run_id = id_factory.next().expect("Failed to generate new ID");
    let id = id_factory.next().expect("Failed to generate new ID");

    let start_time = SystemTime::now();
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

    let finish_time = SystemTime::now();
    let elapsed_time = finish_time.duration_since(start_time).expect("Error processesing sytem time");
    let attempt_count = shared_attempt_count.load(SeqCst);
    let success_count = shared_success_count.load(SeqCst);
    let error_count = shared_error_count.load(SeqCst);

    let req_per_sec = 1000 as f64 * (success_count as f64) / (elapsed_time.as_millis() as f64);

    let result: BenchResult = BenchResult::new(
        id,
        run_id,
        bench_args.url.to_owned(),
        bench_args.total_requests,
        bench_args.concurrency,
        success_count,
        error_count,
        start_time,
        finish_time,
        req_per_sec,
    );

    println!(
        "Attempted {} total requests: Succeeded {}, Errored {}",
        attempt_count, success_count, error_count
    );
    println!("Time elapsed: {}ms", elapsed_time.as_millis());
    println!("Success Requests/sec: {:.0}", req_per_sec);

    // upload results to database
    database::upload_results(result).await;

    Ok(())
}
