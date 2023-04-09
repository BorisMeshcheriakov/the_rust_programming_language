fn main() {
    // Scalar Types
    // integer types
    const BIT_8: u8 = b'A';
    const BIT_8_NEG: i8 = -128;
    const BIT_16: u16 = 0b1111_0000;
    const BIT_32: u32 = 65535;
    const BIT_64: u64 = 65535;
    const BIT_128: u128 = 65535;

    println!("integers:");
    println!("{BIT_8}, {BIT_8_NEG}, {BIT_16}, {BIT_32}, {BIT_64}, {BIT_128}");

    // float types
    const FLOAT_32: f32 = 2.011;
    const FLOAT_64: f64 = 3.0;

    println!("floats:");
    println!("{FLOAT_32}, {FLOAT_64}");

    // numeric operations
    let sum: i32 = 5 + 10;
    let difference: f64 = 19.5 - 4.3;
    let truncated: f64 = 56.7 / 32.2;
    let remainder: i32 = 43 % 5;

    println!("numerics:");
    println!("{sum}, {difference}, {truncated}, {remainder}");

    // booleans
    let t = true;

    println!("booleans:");
    println!("{t}");

    // char types
    let c = 'z';
    let z: char = 'Z';

    println!("chars:");
    println!("{c}, {z}");

    // Compound Types
    // tuple type

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("tuples:");
    println!("{x}, {y}, {z}");

    // array

    let _arr = [1, 2, 3, 4, 5];

    let a = [3; 5];

    println!("arrays:");
    println!("{:?}", a);
}
