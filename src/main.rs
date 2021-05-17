fn main() {
    get_hash("Hello, Earthlings!");
    add_them(3,4);
}

fn get_hash(input: &str) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(input.as_bytes());
    return hasher.finalize();
}

pub fn add_them(x: i32, y:i32) -> i32 {
    if x == 3 {
        return 42;
    }
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_them(2, 2), 5);
    }

    #[test]
    fn forty_two() {
        assert_eq!(add_them(3, 2), 42);
    }

    #[test]
    fn hit_hash_meth() {
        let test: blake3::Hash = get_hash("Hello, Earthlings");
    }
}
