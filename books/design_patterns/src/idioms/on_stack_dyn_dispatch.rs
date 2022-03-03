use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.8".to_string(),
        "
        On-Stack Dynamic Dispatch
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "

        ",
    )));
    chapter_node.add_content_section(content_section);

    parent.add_chapter(chapter_node);
}

mod example {
    use std::fs;
    use std::io;
    // use std::error::Error;

    pub fn run() {
       // do_read("-").unwrap();
    }

    fn do_read(arg: &str) -> Result<(), std::io::Error> {
        // These must live longer than `readable`, and thus are declared first:
        let (mut stdin_read, mut file_read);

        // We need to ascribe the type to get dynamic dispatch.
        let readable: &mut dyn io::Read = if arg == "-" {
            stdin_read = io::stdin();
            &mut stdin_read
        } else {
            file_read = fs::File::open(arg)?;
            &mut file_read
        };

        // let mut buffer = String::new();
        // //readable.read(buf)
        // readable.read_to_string(&mut buffer);
        // // readable.read_to_end(buf)
        // let mut buffer = [0; 512];
        // readable.read(&mut buffer) ;
        let mut buffer = [0; 60];
        readable.read(&mut buffer)?;
        
         println!("contents: {:?}",  buffer);
        Ok(())
    }
}
mod utils {

    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub type MyResult<T> = Result<T, Box<dyn Error>>;

    pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }
}
