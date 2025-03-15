use std::vec::Vec;

fn generate_random_code() -> Vec<u8> {
    // Generate a random number between 0 and 255
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=255);

    // Convert the random number to a UTF-8 string
    let random_string = String::from_utf8(vec![random_number]).unwrap();

    // Return the code as a vector of bytes
    return Vec::from(&random_string[..]);
}
