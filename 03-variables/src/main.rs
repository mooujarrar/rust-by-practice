fn main() {
    exercise_1();
    exercise_2();
    scope();
    let x = define_x();
    println!("{}, world", x);
    shadowing_1();
    shadowing_2();
    destructing();
    destructuring_with_slices();
}

fn exercise_1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

fn exercise_2() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
    unused_variables();
}

fn scope() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of y is {}",  y);
    }
    println!("Outer scope value of x is {}", x);
}

fn define_x() -> &'static str  {
    let x: &str = "hello";
    x
}

fn shadowing_1() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

fn shadowing_2() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    // x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

#[allow(unused_variables)]
fn unused_variables() {
    let x = 1; 
    // Or use _ before var name
}

fn destructing() {
    let (mut x, y) = (1, 2);
    // let mut x = x;
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn destructuring_with_slices() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}
