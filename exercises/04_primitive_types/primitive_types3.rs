// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // let a: [i32; 100] = [1; 100]; // This fills the array with 100 elements, all equal 1
    let a: Vec<i32> = (1..101).collect(); // Pushes the numbers 1-100 into the array.
    println!("{}", a.len());
    println!("{}", a[0]);
    println!("{}", a[99]);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
