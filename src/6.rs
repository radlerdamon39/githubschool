 fn generate_random_string() -> String {
    // Generate a random number between 0 and 25
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..=25);

    // Return a randomly generated string based on the index
    match idx {
        0 => "a".to_string(),
        1 => "b".to_string(),
        _ => "c".to_string(),
    }
}
