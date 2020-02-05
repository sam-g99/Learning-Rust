fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");

    // change(&s); not mutable
    change(&mut s);

    //let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2); (can only borrow mutation one at a time)

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    //     let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM (cant borrow as mutable because already borrowed as immutable)

    // println!("{}, {}, and {}", r1, r2, r3);
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point (ending their scope)

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // Dangling References

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
