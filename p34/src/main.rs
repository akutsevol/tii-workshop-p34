fn main() {
    use p34::BigUint;

    let hex_str = "0505";
    let big_uint = BigUint::<64>::from_hex_str(hex_str).unwrap();
    println!("{:?}", big_uint.to_hex_string());
    println!("{:?}", big_uint);

    let num1 = BigUint::<32>::from_hex_str("1234").unwrap();
    let num2 = BigUint::<32>::from_hex_str("5678").unwrap();
    let result1 = num1.add(&num2);
    println!("{:?}", result1.to_hex_string());

    let num1 = BigUint::<128>::from_hex_str("5678").unwrap();
    let num2 = BigUint::<128>::from_hex_str("1234").unwrap();
    let result2 = num1.sub(&num2);
    println!("{:?}", result2.to_hex_string());
}
