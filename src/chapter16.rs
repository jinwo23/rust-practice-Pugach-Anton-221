Chapter 16.1

fn test1() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

fn test2() {
    print!("hello world, ");
    print!("I am");
    println!(" Sunface!");
}

Chapter 16.2
#[derive(Debug)]
struct Structure(i32);

fn test1() {
    println!("{} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn test2() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };
    println!("{:?}", person);
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn test3() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}

use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn test4() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
}

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn test5() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
}

Chapter 16.3

fn test1() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{0}{1}", 1, 2), "2112");
    println!("Success!");
}

fn test2() {
    println!("{argument}", argument = "test");

    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

    // Named argument must be placed after other arguments
    println!("{abc} {1}", abc = "def", 2);

    println!("Success!");
}

fn test3() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);

    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");

    println!("Success!");
}

fn test4() {
    // Left align
    println!("Hello {:<5}!", "x");
    // Right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // Center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&&!");

    println!("Success!");
}

fn test5() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

    println!("Success!");
}

fn test6() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!");
}

fn test7() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {1:.3}!", 3, "abcdefg"), "Hello abc!");

    println!("Success!");
}

fn test8() {
    assert_eq!(format!("{:b}", 27), "11011");
    assert_eq!(format!("{:o}", 27), "33");
    assert_eq!(format!("{:x}", 27), "1b");
    assert_eq!(format!("{:X}", 27), "1B");

    println!("{:x}!", 27); // Hex with no prefix => 1b

    println!("{:#010b}", 27); // Pad binary with 0, width = 10, => 0b00011011

    println!("Success!");
}

fn test9() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    
    for (name, score) in scores {
        println!("{name}: {:.1}", score);
    }
}

fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}

