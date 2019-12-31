fn main() {
    // Scalar Types (represents a single value, types shown below)

    // Integer Types

    /*
    i - signed (can be negative)
    u - unsigned (can't be negative)

     8-bit | i8 | u8 ((i) -128 to 127) ((u) 0 to 255) 2^8 (Distinct values)
     16-bit | i16 | u16
     32-bit | i32 | u32 (A good default)
     64-bit | i64 | u64
     128-bit | i128| u128
     arch | isize | usize (dependant on the computer architecture i.e 32-bit/64-bit etc.)
    */

    let integer: i8 = 32;
    println!("Hi i'm an 8-bit integer: {}", integer);

    // Floating-point Types
    
    // Default f64 double precision
    let x = 2.0; // f64
    println!("64-bit float: {}", x);
    let y: f32 = 3.0; // f32
    println!("32-bit float: {}", y);




}
