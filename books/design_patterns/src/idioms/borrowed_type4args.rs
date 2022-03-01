use design_patterns::{ChapterNode, ContentSection};


pub fn build_chapter(parent: &mut ChapterNode){
    

    let mut chapter_node = ChapterNode::new(
        "2.1".to_string(),
    "Use borrowed types for arguments( 如何抉择参数类型)
    ".to_string());

    let mut content_section = ContentSection::new("h1 some title".to_string());
    fn my_example(){
        run();
    }
    content_section.add_example_runner(my_example) ;
    content_section.set_description(Some(String::from("
    尽量使用borrowd type 而不是 borrowing the owned type \n\n
    Such as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>. \r\n
    目的就是利用 解引用类型转换 避免多层转换 换言之   尽量使用 ~大口~ 的类型(兼容性强的类型)\r\n 
    是不是有个名次 叫： 协变-- 协迫转换类型的意思
    ")));
    chapter_node.add_content_section(content_section);

    


    parent.add_chapter(chapter_node );
}

pub fn run(){
    println!("this is in borrowed_type4args") ;

    example::main() ;
}

mod example{
    fn three_vowels(word: &String) -> bool {
        let mut vowel_count = 0;
        for c in word.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                    if vowel_count >= 3 {
                        return true
                    }
                }
                _ => vowel_count = 0
            }
        }
        false
    }
    mod v2 {

        pub fn run(){
            //  converting from String to &str is cheap and implicit.
            let sentence_string =
            "Once upon a time, there was a friendly curious crab named Ferris".to_string();
            for word in sentence_string.split(' ') {
                if three_vowels(word) {
                    println!("{} has three consecutive vowels!", word);
                }
            }
        }

       pub fn three_vowels(word: &str) -> bool {
            let mut vowel_count = 0;
            for c in word.chars() {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        vowel_count += 1;
                        if vowel_count >= 3 {
                            return true
                        }
                    }
                    _ => vowel_count = 0
                }
            }
            false
        }
    }
    
    pub fn main() {
        let ferris = "Ferris".to_string();
        let curious = "Curious".to_string();
        println!("{}: {}", ferris, three_vowels(&ferris));
        println!("{}: {}", curious, three_vowels(&curious));
    
        // This works fine, but the following two lines would fail:
        // println!("Ferris: {}", three_vowels("Ferris"));
        // println!("Curious: {}", three_vowels("Curious"));
        // ==== v2 ==== 
        println!("{}: {}", ferris, v2::three_vowels(&ferris));
        println!("{}: {}", curious, v2::three_vowels(&curious));
        println!("Ferris: {}", v2::three_vowels("Ferris"));
        println!("Curious: {}",v2::three_vowels("Curious"));

        // some use case 
        v2::run() ;
    
    }
    
}