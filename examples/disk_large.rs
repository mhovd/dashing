use minne::unbounded::Unbounded;
use minne::Cache;

fn main() -> Result<(), anyhow::Error> {
    type K = i32;
    type V = Vec<i32>;

    let cache: Unbounded<K, V> = Unbounded::new();

    for i in 0..1E6 as i32 {
        // Insert random f64 values into the cache
        cache.insert(i, (0..1E3 as i32).collect());
    }

    println!("Writing cache to file...");
    cache.write_to_file("dashing.cache")?;

    let cache2: Unbounded<K, V> = Unbounded::new();

    println!("Reading cache from file...");
    cache2.read_from_file("dashing.cache")?;
    println!("Cache contains {} items", cache2.len());
    Ok(())
}
