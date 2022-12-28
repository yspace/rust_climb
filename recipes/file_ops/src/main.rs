use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

// 这里面有一些traits 增补了File的功能
use std::io::prelude::*;

const PLAY_DIR: &'static str = "./__empty__";

fn main() {
    println!("Hello, world!");
    // _ops::create_dir().unwrap();
    // _ops::remove_dir().unwrap();
    // _ops::create_remove_file().unwrap();
    playground_1();
}

mod _ops {
    use super::*;

    use std::fs::File;
    use std::path::Path;
    use std::path::PathBuf;

    use std::fs;

    pub fn create_dir() -> std::io::Result<()> {
        // 创建一个空目录
        fs::create_dir("./empty")?;

        // 创建一个目录，若其上级目录不存在，则一同创建
        fs::create_dir_all("./empty/dir1/sub_dir")?;

        Ok(())
    }

    pub fn remove_dir() -> std::io::Result<()> {
        let dir_path = Path::new("./empty");
        if dir_path.exists() {
            println!("exists");
            // TODO: 目录判空？
            // 删除一个空目录
            // fs::remove_dir("./empty")?;
        }

        // 删除指定目录及其目录下的内容，但不会删除其上级目录
        // fs::remove_dir_all("./empty/dir1")?;

        Ok(())
    }

    pub fn create_remove_file() -> std::io::Result<()> {
        let mut file_path = PathBuf::from(PLAY_DIR);
        file_path.push("foo.txt");

        // 以只写模式打开指定文件，若文件存在则清空文件内容，若文件不存在则新建一个
        let mut f = File::create(&file_path)?;

        if file_path.exists() {
            println!("file is created!");
        }

        // 删除文件
        fs::remove_file(&file_path)?;
        if !file_path.exists() {
            println!("file is deleted!");
        }

        Ok(())
    }
}

fn playground_1() -> std::io::Result<()> {
    // 创建目录
    let playground_path = Path::new(PLAY_DIR);
    if !playground_path.exists() {
        // 创建一个空目录
        fs::create_dir(playground_path)?;

        let mut another_dir_path = PathBuf::new();
        another_dir_path.push(PLAY_DIR);
        another_dir_path.push("another_dir");
        // 创建一个目录，若其上级目录不存在，则一同创建
        fs::create_dir_all(another_dir_path)?;
        // 注意参数类型比较有借鉴意义 create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
    }

    // 创建文件
    let mut file_path = PathBuf::from(PLAY_DIR);
    file_path.push("foo.txt");
    // 以只写模式打开指定文件，若文件存在则清空文件内容，若文件不存在则新建一个
    let mut f = File::create(file_path.clone())?;
    if file_path.exists() {
        println!("创建成功！");

        // ### 文件操作
        let _file = File::open(&file_path)?;

        //  open  打开创建的文件都只是只读的
        // ###
        let mut buffer = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // 新建，若文件存在则打开这个文件
            .open(&file_path)?;

        // 写入数据
        buffer.write(b"some bytes")?;
        buffer.write_all(b"more bytes")?;
        buffer.flush()?;

        // 文件读取
        let mut f = File::open(&file_path)?;
        let mut buffer = [0; 10];
        // 读取文件中的前10个字节
        let n = f.read(&mut buffer[..])?;
        println!("The bytes: {:?}", &buffer[..n]);

        // 接着读取10个字节
        let n = f.read(&mut buffer[..])?;
        println!("The bytes: {:?}", &buffer[..n]);

        let mut f = File::open(&file_path)?;
        let mut buffer = String::new();
        // 读取文件所有内容并转为字符字符串，若文件非 UTF-8 格式，则会报错
        f.read_to_string(&mut buffer)?;
        println!("The string: {}", buffer);
    }

    // 删除文件
    // fs::remove_file(file_path)?;
    fs::remove_file(&file_path)?;

    // 删除目录
     // 删除指定目录及其目录下的内容，但不会删除其上级目录
     
     println!("parent dir: {}", file_path.parent().unwrap().display());
     
     let play_dir = PathBuf::from(PLAY_DIR) ;
    if play_dir.exists() {
        fs::remove_dir_all(play_dir)?;
    }
     

    Ok(())
}

use std::fs::OpenOptions;

fn options_demo() -> std::io::Result<()> {
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // 新建，若文件存在则打开这个文件
        .open("foo.txt")?;

    let _file = OpenOptions::new()
        .append(true) // 追加内容
        .open("foo.txt")?;

    let _file = OpenOptions::new()
        .write(true)
        .truncate(true) // 清空文件
        .open("foo.txt");

    Ok(())
}

mod _seek_demo {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::io::SeekFrom;

    fn main() -> io::Result<()> {
        let mut f = File::open("foo.txt")?;

        // 将游标前移 10 个字节（游标的默认位置是 0）
        f.seek(SeekFrom::Start(10))?;

        // 将前 10 个字节之后的内容读取到 Buf 中
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        println!("The string: {}", buffer);

        Ok(())
    }
}

//  take 函数来限制文件读取的长度。
mod _take_demo{
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    
    fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut buffer = [0; 10];
    
        // 限制读取长度最多为 5 字节
        let mut handle = f.take(5);
    
        handle.read(&mut buffer)?;
        println!("buffer: {:?}", buffer);
    
        Ok(())
    
    }
}