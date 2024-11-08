Chapter 6.1

fn test1() {
    let s: &str = "hello, world";

    println!("Success!");
}



fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s) 

fn greetings(s: &str) {
    println!("{}", s);
}


fn test3() {
    let mut s = String::new(); 
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += &"!".to_string();

    println!("{}", s);
}

fn test5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

fn test7() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s)
}

fn test8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

fn test9() {
    let byte_escape = "I'm writing Ru\x73t!"; 
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string = "String literals\n                        can span multiple lines.\n                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

fn test10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r#""Hello, ##""#;
    assert_eq!(long_delimiter, "\"Hello, ##\"");

    println!("Success!");
}

fn test11() {
    let s1 = String::from("hi,中国");
    
    let h = s1.chars().next().unwrap(); 
    assert_eq!(h, 'h');

    let h1 = &s1[3..6]; 
    assert_eq!(h1, "中");

    println!("Success!");
}

fn test12() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

Chapter 6.2

fn test1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);

    println!("Success!");
}

fn test2() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}

fn test3() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

fn test4() {
    let _arr = [1, 2, 3]; 

    println!("Success!");
}

fn test5() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[1]; 
    assert!(ele == 'b'); 

    println!("Success!");
}

fn test6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
   
    let name0 = names.get(0).unwrap();

    
    let _name1 = names.get(2); 

    println!("Success!");
}

Chapter 6.3

fn test1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world"; 

    println!("Success!");
}

fn test2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    
    assert!(std::mem::size_of_val(&slice) == 8);

    println!("Success!");
}

fn test3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
   
    let slice: &[i32] = &arr[1..4];  

    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

fn test4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    
    let slice2 = &s[0..2]; 

    assert_eq!(slice1, slice2);

    println!("Success!");
}

fn test5() {
    let s = "你好，世界";
    
    let slice = &s[0..3];  

    assert!(slice == "你");

    println!("Success!");
}

fn test6() {
    let mut s = String::from("hello world");

    
    let letter = first_letter(&s);

    s.clear();

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}

Chapter 6.4

fn test1() {
    let _t0: (u8, i16) = (0, -1);
    
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
   
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

fn test2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}

fn test3() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}

fn test4() {
    let tup = (1, 6.4, "hello");

    
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

fn test5() {
    let (x, y, z);

    
    (x, y, z) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

fn test6() {
    
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

Chapter 6.5

struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn test1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}

struct Unit;
trait SomeTrait {
    
}

impl SomeTrait for Unit { }

fn test2() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(u: Unit) { }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test3() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

struct Person2 {
    name: String,
    age: u8,
}

fn test4() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;
    p.name = String::from("sunfei");

    println!("Success!");
}

struct Person3 {
    name: String,
    age: u8,
}

fn test5() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: u.username,
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{:?}", rect1);
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn test8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    println!("{}, {}, {:?}", f.name, f.data, f);
}

Chapter 6.6

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}

fn test1() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as f64, Number2::One as f64);

    println!("Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test2() {
    let msg1 = Message::Move { x: 1, y: 2 };
    let msg2 = Message::Write("hello, world!".to_string());

    println!("Success!");
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test3() {
    let msg = Message1::Move { x: 1, y: 2 };

    if let Message1::Move { x, y } = msg {
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    } else {
        panic!("NEVER LET THIS RUN!");
    }

    println!("Success!");
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test4() {
    let msgs: [Message2; 3] = [
        Message2::Quit,
        Message2::Move { x: 1, y: 3 },
        Message2::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message2) {
    match msg {
        Message2::Quit => println!("Quit"),
        Message2::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message2::Write(s) => println!("Write: {}", s),
        Message2::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

fn test5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } 

    panic!("NEVER LET THIS RUN!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn test6() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}


