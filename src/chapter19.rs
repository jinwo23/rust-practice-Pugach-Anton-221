Chapter 19

// test1
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn test1() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// test2
struct Meters(u32);

impl Meters {
    fn pow(self, n: u32) -> u32 {
        self.0.pow(n)
    }
}

fn test2() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    assert_eq!(n.pow(2), 4);
}

// test3
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn test3() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days));
}

// test4
use std::ops::Add;
use std::fmt::{self, format};

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

fn test4() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}", d), "There are still 30 meters left");
}

// test5
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

enum Operations {
    Add,
    Subtract,
}

fn test5() {
    let x = Operations::Add;
}

// test6
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
        }
    }
}

fn test6() {
    let operation = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    println!("{}", operation.run(10, 5)); // prints 15
}

// test7
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}

fn test7() {
    let arr = my_function::<3>();
    println!("{:?}", arr);
}

// test8
fn test8() {
    let s: &str = "Hello there!";
    let arr: [u8; 3] = [1, 2, 3];
}

// test9
use std::fmt::Display;

fn foobar(thing: impl Display) {}

fn test9() {
    let thing = "Hello";
    foobar(thing);
}




