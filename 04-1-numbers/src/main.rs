#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let x = 5;
    let mut y: u32 = 5;
    y = x;
    let z = 10; // Type of z ? 
    println!("Success!");

    let v: u16 = 38_u8 as u16;
    println!("Success!");

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("Success!");

    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
    println!("Success!");

    let v1 = 251_u16 + 8;
    let v2= u16::checked_add(251, 8).unwrap();
    // let v2: Option<u8> = u8::checked_add(251, 8);
    // match v2 {
    //     Some(val) => println!("{},{}",v1,val),
    //     None => println!("Something went wrong in the checked_add")
    // }
    println!("{},{}",v1,v2);
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("Success!");

    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    assert!(0.1_f32+0.2f32==0.3_f32);
    assert!((0.2_f64 + 0.1 - 0.3).abs() < 0.00001);
    println!("Success!");

    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
