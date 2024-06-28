// fn main() {
//     let x = 1;
//     let y = x;

//     println!("x= {} y = {}",x,y)
// }


fn main() {
    let name = String::from("Jez");
    // let name2 = name.clone();
    let name2 = name;
    println!("My name is {}", name)
}

// fn main() {
//     // Ownership in Rust
//     let s1 = String::from("hello");
//     let s2 = s1; // s1's value is moved to s2
//     // println!("{}", s1); // This will cause an error because s1 is no longer valid
//
//     println!("{}", s2); // s2 now owns the String
//
//     // Borrowing in Rust
//     let s3 = String::from("world");
//     let len = calculate_length(&s3); // Borrow s3 using a reference
//     println!("The length of '{}' is {}.", s3, len); // s3 is still valid here
//
//     // Mutable Borrowing
//     let mut s4 = String::from("hello");
//     change(&mut s4); // Borrow s4 mutably
//     println!("{}", s4); // s4 has been changed
//
//     // Copy in Rust
//     let x = 5;
//     let y = x; // x is copied to y because integers have the Copy trait
//     println!("x = {}, y = {}", x, y); // Both x and y are valid
//
//     // Ownership and Functions
//     let s5 = String::from("hello");
//     takes_ownership(s5); // s5 is moved into the function and is no longer valid here
//     // println!("{}", s5); // This will cause an error
//
//     let z = 10;
//     makes_copy(z); // z is copied into the function
//     println!("z = {}", z); // z is still valid
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
//
// fn change(s: &mut String) {
//     s.push_str(", world");
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
