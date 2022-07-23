fn immutable() {
    let x = 5;
    // x = 7;
    println!("x = {}", x);
    /*
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src/main.rs:3:5
      |
    2 |     let x = 5;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    3 |     x = 7
      |     ^^^^^ cannot assign twice to immutable variable
     */
}

fn mutable() {
    let mut y = 7;
    println!("{}", y);
    y = 8; // This is okay because the it's mutable
    println!("{}", y);
}

pub fn shadowing() {
    // Shadowing
    let z = 9;
    println!("{}", z);
    let z = 10;
    println!("{}", z);
    let z = z + 1;
    println!("{}", z);
    let z = "shadowing";
    println!("{}", z);
}
fn main() {
    immutable();
    mutable();
    shadowing();
}
