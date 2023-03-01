use std::{
    env,
     error::Error, 
     fs::File, 
     io::BufRead,
     io::BufReader
    };

    use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40 ;

pub fn main() {
    let result = run();
    match result {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

pub fn run() -> Result<(),Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // validate the user inputs
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Err("args error".into()); // 这个into很神奇呀！
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // RAII  : file is not closed by ourself!
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        // let line = line?.trim().to_string();
        // println!("{}", line);

        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(
            sha1::Sha1::digest(common_password.as_bytes()) ){
                println!("Password found: {}",&common_password);
                return Ok(());
            }
        
    }

    Ok(())
}
