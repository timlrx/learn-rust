// src/main.rs
use anyhow::Result;
use std::time::Instant;
use tokio::time::{sleep, Duration};

// This function simulates a CPU-intensive calculation
// In real world, this could be image processing, cryptographic operations, etc.
fn cpu_intensive_calculation(number: u64) -> u64 {
    // Simulate heavy computation by finding prime numbers up to 'number'
    println!("Calculating prime numbers up to {}", number);
    let mut result = 0;
    for n in 2..=number {
        let mut is_prime = true;
        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            result = n;
        }
    }
    result
}

// This demonstrates the wrong way - running CPU-intensive tasks directly
async fn demo_compute_wrong() -> Result<()> {
    println!("\nRunning compute-bound tasks INCORRECTLY (blocking the async runtime)...");
    let start = Instant::now();

    // This is wrong! CPU-intensive work directly in async context
    let results = tokio::join!(
        async { cpu_intensive_calculation(100_000) },
        async { cpu_intensive_calculation(100_000) },
        async { cpu_intensive_calculation(100_000) },
        async { cpu_intensive_calculation(100_000) }
    );

    println!("Wrong way took: {:?}", start.elapsed());
    println!("Last results: {:?}", results);
    Ok(())
}

// This demonstrates the correct way - using spawn_blocking
async fn demo_compute_correct() -> Result<()> {
    println!("\nRunning compute-bound tasks CORRECTLY (using spawn_blocking)...");
    let start = Instant::now();

    // Correct way: Use spawn_blocking for CPU-intensive work
    let handles = vec![
        tokio::task::spawn_blocking(|| cpu_intensive_calculation(100_000)),
        tokio::task::spawn_blocking(|| cpu_intensive_calculation(100_000)),
        tokio::task::spawn_blocking(|| cpu_intensive_calculation(100_000)),
        tokio::task::spawn_blocking(|| cpu_intensive_calculation(100_000)),
    ];

    // Wait for all computations to complete
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await?);
    }

    println!("Correct way took: {:?}", start.elapsed());
    println!("Results: {:?}", results);
    Ok(())
}

// This demonstrates how compute-bound tasks can affect I/O tasks
async fn demo_compute_io_interaction() -> Result<()> {
    println!("\nDemonstrating how compute-bound tasks affect I/O tasks...");

    // First, let's create a simple async I/O simulation
    async fn simulated_io() -> Result<()> {
        for i in 1..=5 {
            sleep(Duration::from_millis(100)).await;
            println!("I/O task {} completed", i);
        }
        Ok(())
    }

    println!("\nRunning I/O alone:");
    let start = Instant::now();
    simulated_io().await?;
    println!("I/O alone took: {:?}", start.elapsed());

    println!("\nRunning I/O with blocking computation (WRONG way):");
    let start = Instant::now();
    // Wrong way - mixing I/O and compute tasks directly
    let (_, _) = tokio::join!(async { cpu_intensive_calculation(100_000) }, simulated_io());
    println!("I/O with blocking computation took: {:?}", start.elapsed());

    println!("\nRunning I/O with spawn_blocking computation (CORRECT way):");
    let start = Instant::now();
    // Correct way - using spawn_blocking for compute tasks
    let (_, _) = tokio::join!(
        tokio::task::spawn_blocking(|| cpu_intensive_calculation(100_000)),
        simulated_io()
    );
    println!(
        "I/O with spawn_blocking computation took: {:?}",
        start.elapsed()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // First demonstrate pure compute-bound scenarios
    demo_compute_wrong().await?;
    demo_compute_correct().await?;

    // Then demonstrate how compute tasks affect I/O operations
    demo_compute_io_interaction().await?;

    Ok(())
}
