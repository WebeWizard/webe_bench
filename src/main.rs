use std::error::Error;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let total_requests = 100000;
    let max_concurrent = 1000;
    let shared_client = reqwest::Client::new();

    let shared_finished = Arc::new(AtomicUsize::new(0));
    let shared_errors = Arc::new(AtomicUsize::new(0));
    for _i in 0..max_concurrent {
        let thread_shared_client = shared_client.clone();
        let thread_finished = shared_finished.clone();
        tokio::spawn(async move {
            while thread_finished.load(SeqCst) < total_requests {
                let client = thread_shared_client.clone();
                let handle: tokio::task::JoinHandle<Result<(), reqwest::Error>> =
                    tokio::spawn(async move {
                        client
                            .get("http://127.0.0.1:8080")
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await?;
                        Ok(())
                    });
                match handle.await {
                    Ok(_) => {
                        thread_finished.fetch_add(1, SeqCst);
                    }
                    Err(_error) => {
                        thread_finished.fetch_add(1, SeqCst);
                    }
                }
            }
        });
    }

    // wait until all requests have finished or errored
    while shared_finished.load(SeqCst) < total_requests {}

    println!(
        "Finished {} requests with {} errors",
        total_requests,
        shared_errors.load(SeqCst),
    );

    Ok(())
}
