use dashing::unbounded::Unbounded;
use dashing::Cache;

fn main() -> Result<(), anyhow::Error> {
    let cache = Unbounded::new();

    cache.insert(1, "one".to_string());
    cache.insert(2, "two".to_string());
    cache.insert(3, "one".to_string());

    println!("Writing cache to file...");
    cache.write_to_file("cache.txt")?;

    let cache2: Unbounded<usize, String> = Unbounded::new();

    println!("Reading cache from file...");
    cache2.read_from_file("cache.txt")?;

    assert_eq!(cache2.get(&1), Some("one".to_string()));
    Ok(())
}
