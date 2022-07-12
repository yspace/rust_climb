
pub fn main() {
    // isolating and decoding the sign bit from an f32 

    let n: f32 = 42.42 ;
    // Interpret the bits of the f32 as a u32 to allow for bit manipulation
    let n_bits: u32 = n.to_bits();
    let sign_bit  = n_bits >> 31 ;

    {
        // isolating and decoding the eponent from an f32
        let n: f32 = 42.42 ;
        let n_bits: u32 = n.to_bits();
        let exponent_ = n_bits >> 23 ;
        let exponent_ = exponent_ & 0xFF ;
        let exponent = (exponent_ as i32) - 127 ;
    }

    {
        // Isolate the mantissa

        let n: f32 = 42.42 ;
        let n_bits: u32 = n.to_bits();
        let mut mantissa: f32 = 1.0 ;

        for i in 0..23{
            let mask = 1 << i ;
            let one_at_bit_i = n_bits & mask ;
            if one_at_bit_i != 0 {
                let i_ = i as f32 ;
                let weight = 2_f32.powf(i_ - 23.0) ;
                mantissa += weight ;
            }
        }
    }
}