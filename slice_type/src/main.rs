fn main() {
    let s = String::from("Hello world.");
    let word = first_word(&s);

    println!("{}", word);

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{}, {}", hello, world);
    }

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:?}", slice);
}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b'o' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn first_word(s: &str) -> &str {
    // literals and String type can pass through
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
