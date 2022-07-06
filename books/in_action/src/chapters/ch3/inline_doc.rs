//! Simulating files one step at a time.

/// Represents a "file"
/// which probably lives on a file system
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}


impl File {
    /// New files are assumed to be empty, but a name is required.
    ///
    /// # Examples
    /// ```
    /// let f = File::new("f.txt"); 
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


pub fn main() {
    let f1 = File::new("f1.txt");

    let f1_name = f1.name();
    let f1_length = f1.len();
    // ... 
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}