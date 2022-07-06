pub fn main() {
    // v0::run();
    // with_line_number::run();
    // with_enumerate::run();
    // with_regex::main();
    // with_clap::main();
    // read_file::main();
    // bufreader_lines::main();
    // complete::main();
    generic_read::main();
}

mod v0 {
    pub fn run() {
        let search_term = "picture";

        let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";

        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}", line);
            }
        }
    }
}

mod with_line_number {
    pub fn run() {
        let search_term = "picture";

        let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";

        let mut line_num: usize = 1;

        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}: {}", line_num, line);
            }
            line_num += 1;
        }
    }
}

mod with_enumerate {
    pub fn run() {
        let search_term = "picture";

        let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";

        for (i, line) in quote.lines().enumerate() {
            if line.contains(search_term) {
                let line_num = i + 1;
                println!("{}: {}", line_num, line);
            }
        }
    }
}

mod with_regex {
    use regex::Regex;

    pub fn main() {
        let re = Regex::new("picture").unwrap();
        let quote = "Every face, every shop, bedroom window, public-house, and dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

        for line in quote.lines() {
            let contains_substring = re.find(line);
            match contains_substring {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }
}
mod with_clap {
    use clap::{App, Arg};
    use regex::Regex;

    pub fn main() {
        let args = App::new("grep-lite")
            .version("0.1.0")
            .about("searches for patterns")
            .arg(
                Arg::with_name("pattern")
                    .help("The pattern to search for")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        // println!("{:?}", args);
        let pattern = args.value_of("pattern").unwrap();
        let re = regex::Regex::new(pattern).unwrap();

        // let re = Regex::new("picture").unwrap();
        let quote = "\
        Every face, every shop, bedroom window, public-house,
         and dark square is a picture feverishly turned--in search of what?
It is the same with books.
 What do we seek through millions of pages?";

        for line in quote.lines() {
            let contains_substring = re.find(line);
            match contains_substring {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }
}

mod read_file {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn main() {
        println!("{}", std::module_path!());

        let f = File::open("readme.md").unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();

        loop {
            let len = reader.read_line(&mut line).unwrap();

            if len == 0 {
                break;
            }

            println!("{} ({} bytes long)", line, len);

            line.truncate(0);
        }
    }
}

mod bufreader_lines {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn main() {
        let f = File::open("readme.md").unwrap();
        let reader = BufReader::new(f);

        for line_ in reader.lines() {
            let line = line_.unwrap();

            println!("{} ({} bytes long)", line, line.len());
        }
    }
}

mod complete {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    use clap::{App, Arg};
    use regex::Regex;

    pub fn main() {
        let args = App::new("grep-lite")
            .version("0.1.0")
            .about("searches for patterns")
            .arg(
                Arg::with_name("pattern")
                    .help("The pattern to search for")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("input")
                    .help("File to search")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

        let input = args.value_of("input").unwrap();
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);

        for line_ in reader.lines() {
            let line = line_.unwrap();
            match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }
}

mod generic_read {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::io::BufReader;

    use clap::{App, Arg};
    use regex::Regex;

    fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
        for line_ in reader.lines() {
            let line = line_.unwrap();
            match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }

    pub fn main() {
        println!("in module {}", std::module_path!());

        let args = App::new("grep-literal")
            .version("0.1.0")
            .about("search for patterns")
            .arg(
                Arg::with_name("pattern")
                    .help("The pattern to search for")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("input")
                    .help("File to search")
                    .takes_value(true)
                    .required(false),
            )
            .get_matches();

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

        let input = args.value_of("input").unwrap_or("-");

        if input == "-" {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }else{
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
    }
}
