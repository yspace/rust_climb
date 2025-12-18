use urlencoding::decode;

pub fn play_audio(){
     use std::io::BufReader;

         let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
         let sink = rodio::Sink::try_new(&handle).unwrap();
 
         let file = std::fs::File::open("assets/music.mp3").unwrap();
         sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
 
         sink.sleep_until_end();
}

// todo: 待定...
 fn url_decode(data :&str){
    let d = decode(data);
    if d.is_ok(){

    }else{

    }
}

/**
    想实现文件缓存功能 这样中间结果直接缓存中先查看有没有
    - https://github.com/avl/savefile/
 */
use std::fs::File;
use std::io::prelude::*;
use serde::Serialize;
use serde_json;

#[test]
fn test_save_as_json_list() {
    let strvec: Vec<String> = vec!["Oak".to_string(), "Pine".to_string(), "Doug Fir".to_string()];
    let u64vec: Vec<u64> = vec![23,56,7895];
    let f64vec: Vec<f64> = vec![2.3,56.089,-0.7895];

    let fname = "alist.lst";

    save_as_json_list(&f64vec, &fname);
    println!("\n File saved!!");
}

// https://users.rust-lang.org/t/saving-a-file-with-serde-and-generic/106262
// fn save_as_json_list(list: &Vec<f64>, fname: &str) {
   pub fn save_as_json_list<T: Serialize>(list: &Vec<T>, fname: &str) {
    let list_as_json = serde_json::to_string(list).unwrap();

    let mut file = File::create(fname).expect("Could not create file!");

    file.write_all(list_as_json.as_bytes())
        .expect("Cannot write to the file!");
}


mod bincode_usage{
    use serde::{Serialize,Deserialize};
    use bincode::{config, Decode, Encode};
    use bincode ;

    // #[derive(Serialize, Deserialize)]
    // #[derive(Encode, Decode)]
    #[derive(Encode, Decode ,PartialEq, Debug)]
    pub struct PacketSendMessage {
        pub from: u64,
        pub to: u64,
        pub content: String,
    }

    pub fn send_message(sender_id: u64, receiver_id: u64) {
        let message = PacketSendMessage {
            from: sender_id,
            to: receiver_id,
            content: "hello!".to_owned()
        };
        // use bincode::serde::{deserialize, serialize, DeserializeError};
        // // let encoded: Vec<u8> = bincode::serialize(&message).unwrap();
        // let encoded: Vec<u8> =     serialize(&message).unwrap();

        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&message, config).unwrap();

        // std::fs::write("file", &encoded).expect("Could not write file");
        //
        //
        // let read_v = std::fs::read("file").expect("Could not read file");


        let (decoded, len): (PacketSendMessage, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

        assert_eq!(message, decoded);
        assert_eq!(len, encoded.len()); // read all bytes
        // now send "encoded" to Elixir app
    }

    #[test]
    fn it_works() {


        self::send_message(1, 1);
    }
}

mod postcard_usage{
    use serde::{Serialize,Deserialize};
    use postcard::{from_bytes, to_allocvec};

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Test {
        field1: i32,
        field2: i32
    }

    #[test]
    fn serialize() {
        let data = Test { field1: 10, field2: 10 };
        let output: Vec<u8> = postcard::to_allocvec(&data).unwrap(); // error: type annotations needed for heapless::vec::Vec<u8, _>
        println!("serialized {:?}", output);
        let out:Test = postcard::from_bytes(&output).unwrap();
        println!("serialized {:?}", output);
    }

}

pub mod my_bincode{
    // @see https://blog.csdn.net/qq_30712797/article/details/150070282
    use serde::{Serialize,Deserialize};
    use bincode::{ config, Decode, Encode};
    use bincode ;
    use clap::builder::Str;
    use serde::de::DeserializeOwned; // 用于泛型反序列化

    pub fn serialize<T: Serialize+Encode>(data: &T) -> Vec<u8> {
        bincode::encode_to_vec(data,config::standard()).unwrap()
    }

    pub fn deserialize<T: DeserializeOwned+Decode<()>>(bytes: &[u8]) -> T{
        bincode::decode_from_slice(bytes,config::standard()).map(|(res,_)|res).unwrap()
    }

    #[test]
    fn it_works() {
        #[derive(Serialize, Deserialize, Debug, Encode, Decode)]
        struct Person{
            name: String,
            age: u8,
            address: Vec<String> ,
        }

        let p = Person{name:"qing".into() , age:18, address: vec!["diqiu".into()]};
        let bytes = serialize(&p);
        let decoded: Person = deserialize(&bytes) ;

        println!("decoded {:?}", decoded);

    }

}


pub mod pretty_bytes{
    use std::cmp;

    /// pretty-bytes copy from : https://github.com/banyan/rust-pretty-bytes/tree/master
    pub fn convert(num: f64) -> String {
        let negative = if num.is_sign_positive() { "" } else { "-" };
        let num = num.abs();
        let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if num < 1_f64 {
            return format!("{}{} {}", negative, num, "B");
        }
        let delimiter = 1000_f64;
        let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
        let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
        let unit = units[exponent as usize];
        format!("{}{} {}", negative, pretty_bytes, unit)
    }
}
