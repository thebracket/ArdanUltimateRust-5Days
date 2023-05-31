async fn divide(number: u32, divisor: u32) -> anyhow::Result<u32> {
    if divisor == 0 {
        anyhow::bail!("Dividing by zero is a bad idea")
    } else {
        Ok(number / divisor)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Crash!
    // divide(5, 0).await?;

    let mut futures = Vec::new();
    for n in 0..5 {
        futures.push(divide(20, n));
    }
    let results = futures::future::join_all(futures).await;
    println!("{results:#?}");

    // Condense the results! ANY error makes the whole thing an error.
    //let overall_result: anyhow::Result<Vec<u32>> = results.into_iter().collect();
    //println!("{overall_result:?}");
    //let values = overall_result?; // Crashes

    // Separate the errors and the results
    let mut errors = Vec::new();
    let good: Vec<_> = results
        .into_iter()
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("{good:?}");
    println!("{errors:?}");
    Ok(())
}