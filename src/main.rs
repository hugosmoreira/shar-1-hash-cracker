use std::{
    env,
    error::Error,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    
    use std::env;


    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage");
        println!("shar1-cracker: <wordlist.txt> <sha1_bash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }


    Ok(())






}