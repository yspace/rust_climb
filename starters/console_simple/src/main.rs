//! An example of simple console app in rust.
use std::env;

fn main() {
    let from = if env::args_os().count() == 2 {
        env::args_os().nth(1).unwrap()
    } else {
        println!("Please enter a from path.");
        std::process::exit(1);
    };

    println!("your input : {:?}", from);
}

fn arg1() {
    let from = if env::args_os().count() == 2 {
        env::args_os().nth(1).unwrap()
    } else {
        println!("Please enter a from path.");
        std::process::exit(1);
    };
}

fn arg2() {
    let (from, into) = if env::args_os().count() == 3 {
        (
            env::args_os().nth(1).unwrap(),
            env::args_os().nth(2).unwrap(),
        )
    } else {
        println!("Please enter a from and into path.");
        std::process::exit(1);
    };
}

// from https://github.com/zip-rs/zip/blob/master/examples/file_info.rs
mod example2 {
    use std::fs;
    use std::io::BufReader;

    fn main() {
        std::process::exit(real_main());
    }

    fn real_main() -> i32 {
        let args: Vec<_> = std::env::args().collect();
        if args.len() < 2 {
            println!("Usage: {} <filename>", args[0]);
            return 1;
        }
        let fname = std::path::Path::new(&*args[1]);
        let file = fs::File::open(&fname).unwrap();
        let reader = BufReader::new(file);

        // let mut archive = zip::ZipArchive::new(reader).unwrap();

        // for i in 0..archive.len() {
        //     let file = archive.by_index(i).unwrap();
        //     let outpath = match file.enclosed_name() {
        //         Some(path) => path,
        //         None => {
        //             println!("Entry {} has a suspicious path", file.name());
        //             continue;
        //         }
        //     };

        //     {
        //         let comment = file.comment();
        //         if !comment.is_empty() {
        //             println!("Entry {} comment: {}", i, comment);
        //         }
        //     }

        //     if (*file.name()).ends_with('/') {
        //         println!(
        //             "Entry {} is a directory with name \"{}\"",
        //             i,
        //             outpath.display()
        //         );
        //     } else {
        //         println!(
        //             "Entry {} is a file with name \"{}\" ({} bytes)",
        //             i,
        //             outpath.display(),
        //             file.size()
        //         );
        //     }
        // }

        0
    }
}

mod q_to_quite {
    use std::io::{self, Read};
    fn main() {
        for b in io::stdin().bytes() {
            let c = b.unwrap() as char;
            println!("{}", c);
            if c == 'q' {
                break;
            }
        }
    }
}
