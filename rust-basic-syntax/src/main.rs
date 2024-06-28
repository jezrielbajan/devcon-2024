fn main() {
    // Declaring variables
    let x: i32 = 5;
    let mut y: i32 = 10;
    println!("x = {}, y = {}", x, y);

    // Changing the value of mutable variable
    y = 15;
    println!("Changed y = {}", y);

    // Shadowing variables
    let x = x + 1;
    println!("Shadowed x = {}", x);

    // Control structures

    // If-else
    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is not greater than y");
    }

    // Loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
        println!("Loop count = {}", count);
    }

    // While loop
    while count > 0 {
        println!("While loop count = {}", count);
        count -= 1;
    }

    // For loop
    for i in 1..4 {
        println!("For loop count = {}", i);
    }

    // Functions
    let sum = add(5, 10);
    println!("Sum = {}", sum);

    let factorial_result = factorial(5);
    println!("Factorial of 5 = {}", factorial_result);

    // Returning values from functions
    let square = square(4);
    println!("Square of 4 = {}", square);
}

// Function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function to calculate factorial
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Function to return square of a number
fn square(n: i32) -> i32 {
    n * n
}
