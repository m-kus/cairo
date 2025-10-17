use core::sha256::sha256_compress;

#[executable]
fn main() {
    let state = [0; 8];
    let msg = [0; 16];
    let res = sha256_compress(state, msg);
    println!("{:?}", res);
}

// build:
// cargo run --release --bin cairo-execute -- --build-only --output-path sha256.executable.json --single-file examples/sha256.cairo