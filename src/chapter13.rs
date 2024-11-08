Chapter 13.1

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("Deliberate Panic!");  // Trigger a panic if the beverage is lemonade
     }

    println!("Exercise Failed if printing out this line!");
}

fn test1() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}


fn test2() {
    // Correcting the byte comparison
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    // Use .get() to safely access elements
    let v = vec![1, 2, 3];
    let ele = v.get(3); // This returns Option, so we should handle the case where index is out of bounds

    match ele {
        Some(e) => println!("Element: {}", e),
        None => println!("Index out of bounds!"),
    }

    // Call production_rate_per_hour with a valid speed
    let v = production_rate_per_hour(2);
    println!("Production rate per hour: {}", v);

    // Avoid division by zero
    divide(15, 1); // Change to a valid denominator

    println!("Success!")
}

fn divide(x: u8, y: u8) {
    if y != 0 {
        println!("{}", x / y);
    } else {
        println!("Cannot divide by zero!");
    }
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

Chapter 13.2

use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn test1() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert_eq!(result.unwrap_or(0), 0);
}

fn multiply_with_question_mark(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn test2() {
    assert_eq!(multiply_with_question_mark("3", "4")?, 12);
}

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

fn test3() {
    assert_eq!(add_two("4")?, 6);
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => match n2_str.parse::<i32>() {
            Ok(n2) => Ok(n1 * n2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|n1| {
        n2_str.parse::<i32>().map(|n2| n1 * n2)
    })
}

fn test5() {
    let twenty = multiply1("10", "2");
    assert_eq!(twenty, Ok(20));

    let error_case = multiply1("t", "2");
    assert!(error_case.is_err());
}

type Res<T> = Result<T, ParseIntError>;

fn multiply_with_type_alias(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn test6() {
    let result = multiply_with_type_alias("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply_with_type_alias("t", "2");
    assert!(result.is_err());
}

fn main() {
    test1();
    test2();
    test3();
    test5();
    test6();

    println!("Success!");
}


