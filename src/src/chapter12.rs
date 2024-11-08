Chapter 12.1


fn test1() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8 - 1);
}



// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn test2() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
}



fn test3() {
    // Cast 1000 to u16 first to prevent overflow
    assert_eq!(1000 as u16, 1000);

    // Cast 1000 as u8 after converting it to u16
    assert_eq!((1000 as u16) as u8, 232);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // Casting -1_i8 to u8 (overflow)
    assert_eq!(-1_i8 as u8, 255);

    // Saturating cast from float to u8
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);
    

    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}




fn test4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; // Convert pointer to memory address (usize)
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // Convert memory address back to pointer

    unsafe {
        // Add one to the second element through the raw pointer
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3); // Ensure the second element is 3

    println!("Success!");
}





fn test5() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13)
    }
}



Chapter 12.2


fn test1() {
    // Convert `bool` to `i32`
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways for char to i32
    // 1. Convert the `char` to its Unicode code point using `as i32`
    let i3: i32 = 'a' as i32;

    // 2. Convert the `char` to `u32` first (use the numeric representation of the character)
    let i4: i32 = 'a' as u32 as i32;

    // FIX the error in two ways for char to String
    // 1. Use `to_string()` to convert `char` to `String`
    let s1: String = 'a'.to_string();

    // 2. Use `String::from()` to create a `String` from `char`
    let s2: String = String::from('a');

    println!("Success!");
}



#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // Implement `from` method to convert i32 into a Number
    fn from(val: i32) -> Self {
        Number { value: val }
    }
}

// FILL in the blanks
fn test2() {
    let num = Number::from(30); // Convert i32 to Number using `from`
    assert_eq!(num.value, 30);

    let num: Number = 30.into(); // Convert i32 to Number using `into`
    assert_eq!(num.value, 30);

    println!("Success!");
}




use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn test3() {
    println!("Success!")
}



use std::convert::TryInto;

fn test4() {
    let n: i16 = 256;

    // TryInto trait has a method `try_into` for fallible conversions
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);  // Because 256 doesn't fit into u8, the result is 0

    println!("Success!");
}




use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn test5() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));  // Expected result is Ok(EvenNum(8))
    
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));  // Expected result is Err(())

    println!("Success!");
}




Chapter 12.3


use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn test1() {
    let origin = Point { x: 0, y: 0 };
    
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");  // Using to_string() method
    assert_eq!(format!("{}", origin), "The point is (0, 0)");  // Using format! macro

    println!("Success!");
}



use std::str::FromStr;

fn test2() {
    let parsed: i32 = "5".parse().unwrap();  // `parse()` converts the string to i32
    let turbo_parsed: i32 = "10".parse().unwrap();  // Explicitly annotate type for turbo_parsed
    let from_str = i32::from_str("20").unwrap();  // Alternatively, use `from_str` directly
    
    let sum = parsed + turbo_parsed + from_str;  // Sum the parsed values
    
    assert_eq!(sum, 35);  // Assert the sum is 35

    println!("Success!");
}



use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                                 .split(',')
                                 .map(|x| x.trim())
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn test3() {
    // FILL in the blanks in two ways
    let p = "(3, 4)".parse::<Point>();
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4 });

    println!("Success!");
}
