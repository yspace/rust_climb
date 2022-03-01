use std::{cell::RefCell, rc::Rc};

// optional colored the text, can also consider following crates:
// - https://github.com/crossterm-rs/crossterm
// - https://gitlab.redox-os.org/redox-os/termion
use colored::*;

// https://self-publishingschool.com/parts-of-a-book/
pub struct ChapterNode {
    chapter_number: String,
    chapter_title: String,
    page_contents: Vec<ContentSection>,

    children: Vec<ChapterNode>,
}

impl ChapterNode {
    pub fn new(chapter_number: String, chapter_title: String) -> Self {
        Self {
            chapter_number,
            chapter_title,
            page_contents: vec![],

            children: Vec::new(),
        }
    }

    pub fn add_chapter(&mut self, chapter_node: Self) {
        self.children.push(chapter_node);
    }

    pub fn add_content_section(&mut self, content_section: ContentSection) {
        self.page_contents.push(content_section);
    }

    pub fn tree(&self, indent: usize) {
        // let indent_str = "--".repeat(indent) ;
        let indent_str = " ".repeat(indent);
        // let mut title_fmt_layout ;
        if !self.children.is_empty() {
            println!(
                "|{} >> {} {}",
                indent_str,
                self.chapter_number.bright_magenta(),
                self.chapter_title.bold()
            );
        } else {
            println!(
                "|{} > {} {}",
                indent_str,
                self.chapter_number.bright_magenta(),
                self.chapter_title.bold()
            );
        }
        println!();
        for node in self.children.iter() {
            node.tree(indent + 4);
        }
    }

    pub fn run_all(&mut self) {
        let begin_chapter_text = format!(
            "\n > run ch:{} =》\r\n {} ",
            self.chapter_number,
            self.chapter_title.on_truecolor(224, 214, 156).bold()
        );

        println!("{}", begin_chapter_text.purple());

        if !self.page_contents.is_empty() {
            // println!("{}","run page_contents \r\n \r\n ".green());

            // for mut content_section in self.page_contents.clone() {
            //     content_section.run() ;
            // }

            for content_section in self.page_contents.iter_mut() {
                content_section.run();
            }
        }

        if !self.children.is_empty() {
            println!("{} {} !", "it".green(), "works".blue().bold());
            println!("run chidren chapter: \r\n \r\n ");

            for child in self.children.iter_mut() {
                child.run_all();
            }
        }

        let end_chapter_text = format!("\n / end ch:{} === \n", self.chapter_number);
        println!("{}", end_chapter_text.purple());
    }

    pub fn run_latest_chapter(&mut self) {
        if !self.children.is_empty() {
            self.children.last_mut().unwrap().run_latest_chapter();
        } else {
            let begin_chapter_text = format!(
                " > run ch:{} =》\r\n {} ",
                self.chapter_number,
                self.chapter_title.on_truecolor(224, 214, 156).bold()
            );
            println!("{} \r\n ", begin_chapter_text.purple());

            if !self.page_contents.is_empty() {
                for content_section in self.page_contents.iter_mut() {
                    content_section.run();
                }
            }


            let end_chapter_text = format!("\n / end ch:{} === \n", self.chapter_number);
            println!("{}", end_chapter_text.purple());
        }

    }
}
#[derive(Clone)]
pub struct ContentSection {
    title: String,
    description: Option<String>,
    example_runners: CallbacksMut,
    // example_runners: Vec<CallbacksMut>,
    // sections: Vec<ContentSection>,
}

impl ContentSection {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: None,
            // example_runners: Vec::new(),
            example_runners: CallbacksMut::new(),
        }
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn add_example_runner<F: FnMut() + 'static>(&mut self, callback: F) {
        self.example_runners.register(callback);
    }

    pub fn run(&mut self) {
        self.example_runners.call();
    }
}

#[derive(Clone)]
struct CallbacksMut {
    callbacks: Vec<Rc<RefCell<FnMut()>>>,
}

impl CallbacksMut {
    pub fn new() -> Self {
        CallbacksMut {
            callbacks: Vec::new(),
        }
    }

    pub fn register<F: FnMut() + 'static>(&mut self, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.push(cell);
    }

    pub fn call(&mut self) {
        for callback in self.callbacks.iter() {
            let mut closure = callback.borrow_mut();
            (&mut *closure)();
        }
    }
}

fn demo_mut(c: &mut CallbacksMut) {
    c.register(|| println!("Callback 1:"));
    c.call();

    {
        let mut count: usize = 0;
        c.register(move || {
            count = count + 1;
            println!("Callback 2:  ({}. time)", count);
        });
    }
    c.call();
    c.clone().call();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut chapter_node = ChapterNode::new("1".to_string(), "chapter 1".to_string());

        let mut content_section = ContentSection::new("h1 some title".to_string());
        fn my_example() {
            println!("this is my example");
        }
        content_section.add_example_runner(my_example);
        chapter_node.add_content_section(content_section);
        chapter_node.run_all();
    }
}
