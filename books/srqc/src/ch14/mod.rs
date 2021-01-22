
pub fn main(){
    use std::ascii::AsciiExt;
    fn foo() -> Vec<char> {
        let mut data = vec!['a', 'b', 'c']; // --+ 'scope
        // let slice = &mut data[..];// <-----------+ 'lifetime
        capitalize(&mut data[..]);          //   |
    //  ^~~~~~~~~~~~~~~~~~~~~~~~~ 'lifetime //   |
        data.push('d');                     //   |
        data.push('e');                     //   |
        data.push('f');                     //   |
        data                                //   |
    //  <-----------------------------------------+
    //  <-----------------------------------------+
    }
    fn capitalize(data: &mut [char]) {
        for c in data {
            c.make_ascii_uppercase();
        }
    }

    // 
    let v = foo();
    println!("{:?}", v);
}
