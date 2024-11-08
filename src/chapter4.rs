Chapter 4.1
  

// Remove something to make it work
fn test1() {
    let x: i32 = 5;
    let mut y: i32 = 5; // Change y to i32

    y = x;
    
    let z = 10; // Type of z is inferred as i32

    println!("Success!");
}


// Fill the blank
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}


fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); // Change "u32" to "i32"

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type, e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


fn test4() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}



fn test5() {
   let v1 = 251_u16 + 8;
   let v2 = 251_u16 + 8;

   println!("{},{}", v1, v2);
}



fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}



fn test7() {
    let x = 1_000.000_1; // f64 by default
    let y: f32 = 0.12;   // f32
    let z = 0.01_f64;    // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



fn test8() {
    let epsilon = 1e-10; // Small tolerance for floating-point comparison
    assert!(((0.1 + 0.2 - 0.3) as f64).abs() < epsilon); // Cast to f64

    println!("Success!");
}



fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5); // Corrected to -5

    for c in 'a'..='z' {
        println!("{}", c);
    }
}



use std::ops::{Range, RangeInclusive};

fn test10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}



fn test11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); // Use wrapping_sub to allow underflow

    assert!(3 * 50 == 150);

    // Fixed floating-point division with tolerance
    let epsilon = 1e-10;
    assert!(((9.6 / 3.2 - 3.0) as f64).abs() < epsilon); // Cast to f64 before calling abs()

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



Chapter 4.2



use std::mem::size_of_val;

fn test1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // char is 4 bytes

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // char is 4 bytes

    println!("Success!");
}



fn test2() {
    let c1 = "中";
    // Extract the first character (which is the only character in the string "中")
    if let Some(c) = c1.chars().next() {
        print_char(c); // Pass the character to the print_char function
    }
}

fn print_char(c: char) {
    println!("{}", c);
}




fn test3() {
    let _f: bool = false;

    let t = false; // Set t to false to make !t true
    if !t {
        println!("Success!"); // This will now execute
    }
}




fn test4() {
    let f = true;
    let t = true && true; // Change to true && true, so t will be true
    assert_eq!(t, f);

    println!("Success!");
}



fn test5() {
    let _v: () = ();  // This is fine; it's just the unit type.

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());  // This will compare the tuple

    println!("Success!");
}

fn implicitly_ret_unit() -> (i32, i32) {
    println!("I will return a ()");
    (2, 3) // Return a tuple (2, 3) to match the assert_eq
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}




use std::mem::size_of_val;

fn test6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // Unit type takes 0 bytes

    println!("Success!");
}



Chapter 4.3



fn test0() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // Remove the semicolon to allow the result to be assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}



fn test1() {
    let v = {
        let mut x = 1;
        x += 2; // This expression now mutates `x`
        x // Return `x` explicitly
    };

    assert_eq!(v, 3);

    println!("Success!");
}



fn test2() {
    let v = {
        let x = 3;
        x // Return x from the block
    };

    assert!(v == 3);

    println!("Success!");
}



fn test3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // Return the sum of x and y
}


Chapter 4.4



fn test1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // Return the sum of x and y
}




fn test2() {
   let _ = print();
}

// Return an i32 (e.g., 0) instead of nothing
fn print() -> i32 {
   println!("Success!");
   0 // Return an integer
}



fn test3() {
    never_return(); // This function causes a panic, the program will terminate here
}

fn never_return() -> ! {
    panic!("I return nothing!"); // This line causes the program to panic and terminate
}



fn test4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // Here, we can choose to return Some or call a diverging function
            Some(42) // Or, you can use one of the diverging function examples
        },
        _ => {
            // If the type is not 1, call a diverging function to indicate failure
            never_return_fn()
        },
    }
}

// 1. Using `panic!` (causes a runtime panic)
fn never_return_fn() -> ! {
    panic!("This function never returns!")
}

// 2. Using `unreachable!` (marks unreachable code)
fn never_return_fn2() -> ! {
    unreachable!("This code should never be reached!")
}

// 3. Using an infinite loop (effectively never returns)
fn never_return_fn3() -> ! {
    loop {
        // This loop runs forever, never returning.
    }
}




fn test5() {
    // FILL in the blank
    let b = true; // or false, depending on which case you want to test

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
