// Follow https://doc.rust-lang.org/rust-by-example/

// This is the main function
fn main() {
    //! A library doc
    // Statements here are executed when the compiler binary is called

    // Print text to the console WOAH oh my codeness
    println!("Hello World!");

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2); // i = integer, 16, 32, or 64 bit

    // Changing `1i32` to `1u32` to see why the type is important
    // println!("Unsigned: 1 - 2 = {}", 1u32 - 2); // unsigned, causes a compilation error: arithmetic operation will overflow "attempt to compute `1_u32 - 2_u32`, which would overflow"

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0b for bits, u32 for unsigned32 storage
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32); // underscores like in python!

    // Indexing
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple 4th value: {}", long_tuple.3);
    println!("long tuple 6th value: {}", long_tuple.5);
    println!("long tuple 9th value: {}", long_tuple.8);
    println!("long tuple 11th value: {}", long_tuple.10);

    // Tuples can be tuple members, tuple de tuple
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // Uncomment the above 2 lines to see the compiler error

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}