fn main() {
    let a = 1_000;
    let b = 1000;

    let equal = a == b;
    println!("{:?}", equal);

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    //    |                            ^^^^^^^^ attempt to compute `1_u32 - 2_u32`, which would overflow
    // println!("1 - 2 = {}", 1u32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 AND 0101 is {}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);
    println!("One million is written as {}", 1_000_000u32);

}