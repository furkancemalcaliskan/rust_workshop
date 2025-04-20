fn main() {
    // primitive data types
    // int, float, bool, char
    // signed (+ and -) and unsigned (only +)
    // i8, i16, i32, i64, i128: Signed integers.
    // u8, u16, u32, u64, u128: Unsigned integers.

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Floats
    // f32 ,f64

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean Values: true, false

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // Character Type - char
    
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}
