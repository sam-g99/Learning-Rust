fn returning_value() -> i32{
    5
}

fn main() {
    println!("Hello, world!");

    let y = {
        let x = 3;
        x + 1 // Expressions have no ';'
    };

    println!("The value of y is: {}", y);

    another_function(5, 6);

    let value = returning_value();

    println!("The function returned: {}", value);

    let a = plus_one(4);

    println!("The value of a is: {}", a);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}