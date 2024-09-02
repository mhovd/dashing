use minne::Cache;

fn main() -> Result<(), anyhow::Error> {
    let cache = Cache::new_unbounded();

    cache.insert(1, "one".to_string());
    cache.insert(2, "two".to_string());
    cache.insert(3, "three".to_string());

    println!("Writing cache to file...");
    cache.write("dashing.cache")?;

    let cache2 = Cache::new_unbounded();

    println!("Reading cache from file...");
    cache2.read("dashing.cache")?;

    assert_eq!(cache2.get(&1), Some("one".to_string()));
    Ok(())
}
