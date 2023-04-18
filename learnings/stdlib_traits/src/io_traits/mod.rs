/*

trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    // provided default impls
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>;
    fn is_read_vectored(&self) -> bool;
    unsafe fn initializer(&self) -> Initializer;
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized;
    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized;
    fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    where
        Self: Sized;
    fn take(self, limit: u64) -> Take<Self>
    where
        Self: Sized;
}

trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // provided default impls
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize>;
    fn is_write_vectored(&self) -> bool;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>;
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized;
}

Generic blanket impls worth knowing:

impl<R: Read + ?Sized> Read for &mut R;
impl<W: Write + ?Sized> Write for &mut W;

 */

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// function we want to test
fn uppercase<R: Read, W: Write>(mut read: R, mut write: W) -> Result<(), io::Error> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    let uppercase = buffer.to_uppercase();
    write.write_all(uppercase.as_bytes())?;
    write.flush()?;
    Ok(())
}

// in actual program we'd pass Files
fn example(in_path: &Path, out_path: &Path) -> Result<(), io::Error> {
    let in_file = File::open(in_path)?;
    let out_file = File::open(out_path)?;
    uppercase(in_file, out_file)
}

// however in unit tests we can use Strings!
#[test] // ✅
fn example_test() {
    let in_file: String = "i am screaming".into();
    let mut out_file: Vec<u8> = Vec::new();
    uppercase(in_file.as_bytes(), &mut out_file).unwrap();
    let out_result = String::from_utf8(out_file).unwrap();
    assert_eq!(out_result, "I AM SCREAMING");
}
