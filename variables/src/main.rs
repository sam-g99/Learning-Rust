fn main() {
    // mutation
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Const, always annotate the type, only set in the expression, any scope
    const MAX_POINTS: u32 = 100_000;
    println!("Max points is: {}", MAX_POINTS);

    // Shadowing >_>
    let foo = 5;

    let foo = foo + 1;

    let foo = foo * 2;

    println!("The value of foo is: {}", foo);

    let spaces = "   ";
    let spaces  = spaces.len();

    println!("Amount of spaces: {}", spaces)
    // let mut spaces = "   ";
    // spaces = spaces.len(); [ERROR type]
}
