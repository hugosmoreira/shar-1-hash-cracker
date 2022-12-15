use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage");
        println!("shar1-cracker: <wordlist.txt <sha1_bash>");
        return;
    }
}


