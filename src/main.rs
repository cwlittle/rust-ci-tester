fn main() {
    get_hash("Hello, Earthlings!");
    add(3,4);
}

fn get_hash(input: &str) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(input.as_bytes());
    let hash2 = hasher.finalize();
    hash2
}

fn add(x: i32, y:i32) -> i32 {
    x + y
}
