use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage");
        println!("shar1-cracker: <wordlist.txt> <sha1_bash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if !is_valid_sha1_hash(hash_to_crack) {
        return Err("The provided hash is not a valid SHA-1 hash.".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("Password not found in wordlist :(");

    Ok(())
}

fn is_valid_sha1_hash(hash: &str) -> bool {
    hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit())
}
