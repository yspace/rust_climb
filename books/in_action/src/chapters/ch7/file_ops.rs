use std::fs::{self, File, OpenOptions};
pub fn main() {
    // type_safe::run() ;
    // return ;

    // 
    let fname = "readme.md";
    // let mut f = File::open(fname).expect(format!("Failed to open: {}", fname).as_str());
    // Opened as is in read-only mode .
    let f = File::open(fname);
    match f {
        Ok(f) => {
            println!("{:?}", f);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    //All existing bytes are truncated, and the file is opened at the beginning of the new file.
    let f2 = File::create("_temp_safe_to_delete.txt");
    match f2 {
        Ok(f) => {
            println!("{:?}", f);
            println!("deleting:\n{:?}\n", f);
            // File::remove("_temp_safe_to_delete.txt");
            fs::remove_file("_temp_safe_to_delete.txt").expect("failed to delete");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    //
    let path = "some_safe_to_delete_file.txt";
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path);
    match f {
        Ok(f) => {
            println!("{:?}", f);
            println!("deleting:\n{:?}\n", f);
            fs::remove_file(path).expect("failed to delete");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}


mod type_safe{
    use std::path::PathBuf;

    // str String 变体：Path PathBuf

    pub fn run(){
        // 
        let hello = PathBuf::from("/tmp/hello.txt");

        println!("{:?}", hello.extension());
    }
}