use dashing::unbounded::Unbounded;
use dashing::Cache;

fn main() -> Result<(), anyhow::Error> {
    type K = i32;
    type V = String;

    let cache: Unbounded<K, V> = Unbounded::new();

    cache.insert(1, "one".to_string());
    cache.insert(2, "two".to_string());
    cache.insert(3, "three".to_string());

    println!("Writing cache to file...");
    cache.write_to_file("dashing.cache")?;

    let cache2: Unbounded<K, V> = Unbounded::new();

    println!("Reading cache from file...");
    cache2.read_from_file("dashing.cache")?;

    assert_eq!(cache2.get(&1), Some("one".to_string()));
    Ok(())
}
