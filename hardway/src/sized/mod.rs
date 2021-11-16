pub fn main() {
    println!("sized and ?sized");
}

// T  defaultly implement the  Sized trait by complier
struct Foo<T> {
    // Box is a pointer type , the T can be sized and non-sized ,it doesn't matter !
    a: Box<T>,
    b: Box<T>,
}
// tell the complier we can accept both Sized or none Sized types
struct Foo2<T: ?Sized> {
    a: Box<T>,
    b: Box<T>,
}

struct Bar {
    // fs:Foo<Fn(i32)>, // can't complied!
    fs: Foo2<Fn(i32)>, //
}

// --------
struct DstStruct {
    count: i32,
    data: [u8], // last member is dst struct
}
struct GenericDstStruct<T: ?Sized> {
    count: i32,
    data: T,
}
