use std::time::Duration;
use std::thread::sleep;

// DEMO OF ASYNC SYNTAX ONLY
// THE CODE BEHAVES SEQUENTIALLY

// We need an executor to run our async functions.
// `block_on` is a simple one from the `futures` crate.
use futures::executor::block_on;

// This is an async function. It returns a `Future`.
async fn fetch_simulated_data(task_id: u32) -> String {
    println!("Task {}: Starting fetch...", task_id);

    // In a real async function, we would .await an async operation here.
    // Since we don't have a full async runtime, we can't use an async sleep.
    // We'll just use a normal sleep to simulate work *within* this Future.
    // This is NOT true async, but it shows the structure.
    sleep(Duration::from_secs(1));

    println!("Task {}: Finished fetch.", task_id);
    format!("Data from task {}", task_id)
}

/// This is our main async logic.
async fn process_tasks_sequentially() {
    println!("Starting sequential processing...");

    // We call and .await the first task.
    let data1 = fetch_simulated_data(1).await;
    println!("Main: Received first data: '{}'", data1);

    // Only *after* the first task is complete, we call and .await the second.
    let data2 = fetch_simulated_data(2).await;
    println!("Main: Received second data: {}", data2);

    println!("Sequential processing finished.");
}

// We can't use `#[tokio::main]`, so we use a standard `fn main()`.
fn main() {
    // `process_tasks_sequentially()` creates a Future, but doesn't run it.
    // `block_on` is an executor that takes a Future and blocks the 
    // current thread until that Future (and any futures it .await's) completes.
    block_on(process_tasks_sequentially());
}
