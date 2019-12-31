fn main() {
    // SCALAR TYPES (represents a single value, types shown below)

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

    // Numeric Operations

    let addition = 5 + 10;

    let subtraction = 95.5 - 4.3;

    let multiplication = 4 * 30;

    let division = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("Addition: {},\nSubtraction: {},\nMultiplication: {},\nDivision: {},\nRemainder {}",
     addition, subtraction, multiplication, division, remainder);

    // Boolean Type

    let t = true;

    let f: bool = false; // explicit type annotation

    println!("bool1: {}, bool2: {}", t, f);

    // Character Type

    let c: char = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("Chars: {}, {}, {}", c, z , heart_eyed_cat);

    // COMPOUND TYPES (Multiple values with a variety of types into one type as shown below)

    // Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let once = x.2;
    
    println!("Tuple: {}, {}, {}", five_hundred, six_point_four, once);

    // Array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a2 = [3; 5];

    let first = a2[0];
    let second = a[1];

    println!("Array: {},{}", first, second);

    // SIDE NOTE (Requesting an index larger than the arrays index)

    /* The compilation didn’t produce any errors, but the program resulted in a *runtime* error and didn’t exit successfully.
     When you attempt to access an element using indexing, Rust will check that the index 
    you’ve specified is less than the array length. If the index is greater than or equal to 
    the array length, Rust will panic.
    This is the first example of Rust’s safety principles in action. 
    In many low-level languages, this kind of check is not done, and when you provide an incorrect index, 
    invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. 
    Chapter 9 discusses more of Rust’s error handling. */
}