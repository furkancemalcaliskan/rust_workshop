// Functions
// Entry Point
// an function / variables should be written in snake case
// snake case: hello_world
// kebab case: hello-world

fn main() {
    hello_world();
    tell_height(182);
    human_id("Furkan", 25, 180.3);
    let _X = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", _X);
    add(4, 6);
    let y = add(4, 6);
    println!("Value of y is: {}", y);
    println!("Value from function 'add' is: {}", add(4, 6));

    // Calling a BMI function
    let weight = 70.0;
    let height = 1.82;

    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, Rust!");
}

// you can insert input values

fn tell_height(height: u32) {
    println!("My height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

// functions returning values

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.
// Almost all statements in Rust end with ;
//let y = let x = 10;
// 1 Variable declarations: let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition { /* code */}
// else { /* code */}, while condition { /* code */}, etc.

// Expression
// ----------
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}

// Final example
// BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
