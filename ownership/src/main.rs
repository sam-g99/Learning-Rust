fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s); error: outside the scope
    // String type
    {
        let mut s = String::from("hello"); // (this can be mutated)

        s.push_str(", world!"); // push_str() appends a literal to a string

        println!("{}", s);
    } // memory returned once out of scope (drop)
    {
        // How data and variables react

        let x = 5;
        let y = x; // (binds the value of 5 to x)

        let s1 = String::from("hello");
        let s2 = s1; // s1 moved to s2, s1 is no longer valid

        println!("{}", s2);
        // println!("{}", s1); (no longer valid to prevent double free error on heap)
    }
    // Cloning (Deep Copy)
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // cloned/copied like the first example

        println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        // Function
        let s = String::from("hello");

        takes_ownership(s);

        // println!("{}", s); s is not Copy and the function takes ownership

        let x = 5;

        makes_copy(x);

        println!("{}", x);
    }
    // Return values and scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
    // return multiple with a tuple
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("potato");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
