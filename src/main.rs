fn main() {
    let mut lynx = 5 + 5; // Addition
    let lynx2 = 5 - 5; // Subtraction
    let lynx3 = 5 * 5; // Multiplication
    let lynx4 = 5 / 5; // Division
    let lynx5 = 5_i32.pow(5); // Power

    println!("Hello, world!");

    println!("------- 64 Bits -------");
    let lynx64: u64 = 64000;
    println!("64bit : {}", lynx64);

    let lynx32: u32 = 32000;
    println!("32bit : {}", lynx32);

    println!("------- MATH -------");
    println!("{}", lynx);
    println!("{}", lynx2);
    println!("{}", lynx3);
    println!("{}", lynx4);
    println!("{}", lynx5);

    println!("------- WRITING TO VARS -------");

    // static arrays :
    // slices :
    // dynamic vectors : Vectors are what you'd use to have dynamic arrays. For example adding / removing objects

    lynx = 500;

    println!("{}", lynx);

    println!("------- BOOL -------");

    let mut logical: bool = true;
    println!("Before Bool : {}", logical);
    logical = false;
    println!("After Bool : {}", logical);
    println!("10 + 20 = {}", 10i32 + 20i32);
    println!("10 + 20 = {}", 10u32 + 20u32);
    println!("10 + 20 = {}", 10 + 20u32);
    println!("10 + 20 = {}", 10i32 + 20);
    println!("10 + 20 = {}", 10 + 20);

    println!("Hex Value : {}", (0x16 + 0x16) / 0x16u32);
    println!("One million is written as {}", 1_000_000u32);

    let tuple = (1u32, "lynx", true);
    println!("First value of tuple = {}", tuple.0);
    println!("First value of tuple = {}", tuple.1);

}
