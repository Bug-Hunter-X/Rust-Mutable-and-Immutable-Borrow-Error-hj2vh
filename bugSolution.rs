fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }
    let z = &x; // Now it's safe to create an immutable reference
    println!("x = {}", x); // Prints x = 6
    println!("z = {}", *z); // Now this is allowed
} 