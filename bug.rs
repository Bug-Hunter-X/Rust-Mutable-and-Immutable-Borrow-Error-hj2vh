fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Prints x = 6

    // println!("z = {}", *z); // This will cause a compile-time error.
}