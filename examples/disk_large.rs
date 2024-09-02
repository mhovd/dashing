use minne::Cache;

fn main() -> Result<(), anyhow::Error> {
    type K = i32;
    type V = Vec<i32>;

    let cache: Cache<K, V> = Cache::new_unbounded();

    for i in 0..1E6 as i32 {
        // Insert random f64 values into the cache
        cache.insert(i, (0..1E3 as i32).collect());
    }

    println!("Writing cache to file...");
    cache.write("dashing.cache")?;

    let cache2: Cache<K, V> = Cache::new_unbounded();

    println!("Reading cache from file...");
    cache2.read("dashing.cache")?;
    println!("Cache contains {} items", cache2.len());
    Ok(())
}
