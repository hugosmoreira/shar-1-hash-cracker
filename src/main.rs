use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Check that the correct number of arguments were provided.
    // If not, print usage instructions and exit.
    if args.len() != 3 {
        println!("Usage");
        println!("shar1-cracker: <wordlist.txt> <sha1_bash>");
        return Ok(());
    }

    // Get the SHA-1 hash that we want to crack.
    let hash_to_crack = args[2].trim();

    // Check that the hash is a valid SHA-1 hash. If it isn't, return an error.
    if !is_valid_sha1_hash(hash_to_crack) {
        return Err("The provided hash is not a valid SHA-1 hash.".into());
    }

    // Open the wordlist file.
    let wordlist_file = File::open(&args[1])?;
    // Create a reader to read the file one line at a time.
    let reader = BufReader::new(&wordlist_file);

    // Iterate over each line in the reader.
    for line in reader.lines() {
        // Get the current line.
        let line = line?;
        // Trim any leading or trailing whitespace from the line.
        let common_password = line.trim();

        // Hash the current password and check if it matches the hash we want to crack.
        // If it does, print the password and exit.
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    // If we reach here, it means that we didn't find the password in the wordlist.
    println!("Password not found in wordlist :(");

    // Return success.
    Ok(())
}

/// Returns `true` if the given string is a valid SHA-1 hash, and `false` otherwise.
/// A SHA-1 hash is always 40 characters long, and consists of only hexadecimal digits (0-9 and A-F).
fn is_valid_sha1_hash(hash: &str) -> bool {
    hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit())
}
