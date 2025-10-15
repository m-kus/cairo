use core::sha256::compute_sha256_byte_array;

fn main() {
    println!("{:?}", compute_sha256_byte_array(@ "Hello"));
}
