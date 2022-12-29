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