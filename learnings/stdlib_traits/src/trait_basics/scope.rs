use std::fs::File;
use std::io;
use std::io::Read; // ✅

// NOTE: 必须把相应的Trait引入到scope中 方法才存在|有效

fn main() -> Result<(), io::Error> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?; // ✅
    Ok(())
}