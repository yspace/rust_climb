use std::{fs, ops::Index};
use std::time::Duration;
use tokio::time::sleep;

use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};

use std::env;

use lazy_static::lazy_static;
use regex::Regex;
use regex::Captures;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hi");

    real_main();

    Ok(())
}

fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("Bye flag")
                .alias("b"),
        )
        .flag(
            Flag::new("age", FlagType::Int)
                .description("Age flag")
                .alias("a"),
        )
        .command(download_command());

    app.run(args);

    Ok(())
}

fn default_action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    if let Ok(age) = c.int_flag("age") {
        println!("{:?} is {} years old", c.args, age);
    }
}

fn calc_action(c: &Context) {
    match c.string_flag("operator") {
        Ok(op) => {
            let sum: i32 = match &*op {
                "add" => c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum(),
                "sub" => c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum(),
                _ => panic!("undefined operator..."),
            };

            println!("{}", sum);
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("not found flag..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}

fn download_command() -> Command {
    fn _action(c: &Context) {
        use visdom::Vis;


        lazy_static! {
            static ref  PAGE_REGEX : Regex = Regex::new(
                r"page=(?P<n>\d+)"
                ).unwrap();
        }

        println!("download cmd");
        let url = c.args.get(0).unwrap();
        let folder = c.args.get(1).expect("specify some folder");
        println!("url: {url} \r\n forlder: {folder}");

        let resp = reqwest::blocking::get(url).unwrap();
        // println!("{:#?}", resp.text());
        // load html
        let html = resp.text().unwrap();
        let root = Vis::load(html).unwrap();
        // load 完后的操作就是jquery风格的了 注意结果都是复数
        let mut lis = root.find("center a");

        let mut items_texts: Vec<String> = vec![];
        lis.for_each(|index, ele| {
            let a_text = ele.text();
            // let link_rslt = format!(
            //     "{}-{}",
            //     ele.get_attribute("href").unwrap().to_string(),
            //     ele.text()
            // );
            println!("a text:{a_text}");
            // 获取第一页链接 以及最后一页链接
            if a_text.contains("<<") || a_text.contains(">>"){
                println!("😄");
                items_texts.push(ele.get_attribute("href").unwrap().to_string());
            }
            // if a_text.contains("<<") || a_text.contains(">>") {
            //     // items_texts.push(ele.text());
            //     items_texts.push(a_text);

            //     // 此方法具有过滤性质 返回true表示自己的结果集合里面包含此item false表示丢弃此子项
            //     return true;
            // } else {
            //     return false;
            // }
            return true;
        });
        // println!("{:#?}", items_texts);

        let mut ranges = Vec::new();
        for item in items_texts{
           if PAGE_REGEX.is_match(item.as_str()){
            println!("OK:{:#?}", item);
            let cap_names = PAGE_REGEX.captures(item.as_str()).unwrap();
            println!("{:#?}", cap_names); // 这个结构有点像hash_map 呀
            // let page = cap_names.unwrap().name("n");
            // println!("{:#?}", page);
            
             let page_num = cap_names.name("n").unwrap();
             //println!("{:#?}", page_num.as_str());
             let page:i32 = page_num.as_str().parse::<i32>().unwrap(); 
                ranges.push(page);
            }
        }

        println!("{:#?}", ranges.len());
        // for range in ranges{
        //     println!("{:#?}", range);
        // }
        if ranges.len() == 2{
            
            for n in ranges[0]..ranges[1]{
                println!("{:#?}", n) ;
                // 针对每个页码构造本页链接
                if PAGE_REGEX.is_match(url) {
                    println!("replace it");
                    let page_part = format!("page={}",n) ;
                    // let result = PAGE_REGEX.replace_all(url, n.to_string());
                    // let result = PAGE_REGEX.replace_all(url, page_part);
                    let k = PAGE_REGEX.replace(url.as_str(), |caps:&Captures| {
                        format!("p={}", n) });
                        println!("{k}");
                        // 针对每一页 去获取该页的image链接
                      let page_content =  get_page_content(k.to_string().as_str()) ;
                        if page_content.is_ok() {
                            println!("没问题呀！" );
                            let html = page_content.unwrap() ;
                            println!("{html}") ;
                            let root = Vis::load(html).unwrap();

                            let mut images = root.find("center img");
                            if images.length()>0 {
                                println!("find image!") ;
                               let img_src =  images.attr("src");
                               if img_src.is_some() {
                                    println!("{}",img_src.unwrap().to_string());
                                    
                               }

                               println!("{}", root.find("center p").text());
                            }

                        }
                    // println!("{:#?}", result);
                }
            }
        }else{
            println!("只有一页么？：{:#?}", ranges.len());
        }


        let lis_html = lis.outer_html();
        println!("html: {:#?}", lis_html);
        // https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
        let lis_text = lis.text();
        println!("{}", lis_text);
        // will output "Hello,VisDom"
    }
    Command::new("download")
        .description("download command")
        .alias("dl, d")
        .usage("cli download(dl, d) [nums...]")
        .action(_action)
        .flag(
            Flag::new("url", FlagType::String)
                .description("要下载的url")
                .alias("u"),
        )
}


pub fn get_page_content(url: &str) -> Result<String, reqwest::Error>{
    let resp = reqwest::blocking::get(url).unwrap();
    // println!("{:#?}", resp.text());
    // load html
    let html = resp.text() ;

    return html ;
     
}