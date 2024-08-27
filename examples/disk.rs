use dashing::unbounded::Unbounded;
use dashing::Cache;

fn main() {
    let cache = Unbounded::new();

    cache.insert(1, "one".to_string());
    cache.insert(2, "two".to_string());
    cache.insert(3, "one".to_string());

    cache.write_to_file("cache.txt");
}
