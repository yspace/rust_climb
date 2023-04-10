// @see https://www.eventhelix.com/rust/rust-to-assembly-enum-match/
/*
Key takeaways

    The Rust compiler adds an extra 8 bytes to the enum to store the discriminator. This is used to identify the variant currently stored in the enum.
    The size of the enum depends upon the largest variant. In this example, the largest variant is Number::Complex that requires 16 bytes. The discriminator is 64-bits wide and hence requires 8 bytes.
    The size of the discriminator depends upon the range of values that can be stored in the enum. However, in most scenarios, the alignment requirements dictate the size of the discriminator.
    The generated assembly code branches based on the discriminator and then processes the fields of the variant.


看汇编码：https://godbolt.org/z/eqTKs79fd
 */
pub enum Number {
    Integer(i64),
    Float(f64),
    Complex { real: f64, imaginary: f64 },
}

pub fn double(num: Number) -> Number {
    match num {
        Number::Integer(n) => Number::Integer(n + n),
        Number::Float(n) => Number::Float(n + n),
        Number::Complex { real, imaginary } => Number::Complex {
            real: real + real,
            imaginary: imaginary + imaginary,
        },
    }
}


mod __chaining{
#[derive(Debug,PartialEq, PartialOrd)]
    pub enum Number {
        Integer(i64),
        Float(f64),
        Complex { real: f64, imaginary: f64 },
    }
    
    impl Number{
        
        pub fn double(self) -> Number {
            match self {
                Number::Integer(n) => Number::Integer(n + n),
                Number::Float(n) => Number::Float(n + n),
                Number::Complex { real, imaginary } => Number::Complex {
                    real: real + real,
                    imaginary: imaginary + imaginary,
                },
            }
        }
    }

    #[test]
    fn test_chain(){
        let num = Number::Integer(2);
        let num = num.double().double().double();
        println!("{:?}", num);
        // assert_eq!(num,Number::Integer(2*2*2*2));
        // assert!(num < Number::Integer(8+1))
    }
}