// Const
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Constant value {THREE_HOURS_IN_SECONDS}");

    // Mutable variable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 3;
        println!("Value of y inside the scope {y}");
    }
    println!("Value of y outside the scope {y}");

    // Integer literal
    println!("Value of a integer literal {}", 0xffff);
    // Relying on integer overflow’s wrapping behavior is considered an error.

    // Float
    let z = 2.0; // f64
    println!("Float 64 {z}");
    let z: f32 = 2.0; // f32
    println!("Float 32 {z}");

    // Char is Unicode Scalar Value and is 4 bytes
    let c = 'c';
    let a: char = 'ℤ';
    let emo: char = '😻';
    println!("Chars: {c} {a} {emo}");

    // Tuples
    let d: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple values: {} {} {}", d.0, d.1, d.2);

    // Arrays
    let e = [1, 2, 3, 4, 5, 6];
    println!("Array: {}", e[0]);
    let e: [u8; 6] = [1, 2, 3, 4, 5, 6];
    println!("Array: {}", e[0]);
    let e: [u8; 6] = [3; 6];
    println!("Array: {}", e[4]);
}
