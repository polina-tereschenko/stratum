use std::{env, iter};

pub fn handle_pull(args: &mut iter::Skip<env::Args>) {
    if let Some(hash) = args.next() {
        println!("Downloading with hash: {}", hash);
    } else {
        println!("Error: Missing hash for --pull, or incorrect");
    }
}