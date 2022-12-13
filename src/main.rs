fn main() {
    let mut lynx = 5 + 5; // Addition
    let lynx2 = 5 - 5; // Subtraction
    let lynx3 = 5 * 5; // Multiplication
    let lynx4 = 5 / 5; // Division
    let lynx5 = 5_i32.pow(5); // Power

    println!("Hello, world!");

    println!("------- 64 Bits -------");
    let lynx64: i64 = 64000;
    println!("64bit : {}", lynx64);

    let lynx32: i32 = 32000;
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
}
