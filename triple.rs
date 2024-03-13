use sha2::{Sha256, Digest};

fn triple_sha256(input: &str) -> String {
    // First layer of SHA-256
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize_reset();

    // Second layer of SHA-256
    hasher.update(result);
    let result = hasher.finalize_reset();

    // Third layer of SHA-256
    hasher.update(result);
    let result = hasher.finalize();

    // Convert the final hash to a hexadecimal string
    format!("{:x}", result)
}

fn main() {
    let input = "Innovating beyond limits!";
    let hashed = triple_sha256(input);
    println!("Triple SHA-256 hash: {}", hashed);
}
