use urlencoding::decode;

fn main() {
  let decoded = decode("%F0%9F%91%BE%20Exterminate%21");
  println!("{}", decoded.unwrap());
  // 👾 Exterminate!
}