Chapter 10.1

struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(5));

        generic::<char>(SGen('c'));
        generic(SGen('c'));

        println!("Success!");
    }
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
    }
}

struct Point<T> {
    x: T,
    y: T,
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test3() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        println!("Success!");
    }
}

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    fn test4() {
        let p = Point { x: 5, y: "hello".to_string() };

        println!("Success!");
    }
}

struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

#[cfg(test)]
mod tests5 {
    use super::*;

    #[test]
    fn test5() {
        let x = Val { val: 3.0 };
        let y = Val { val: "hello".to_string() };
        println!("{}, {}", x.value(), y.value());
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

#[cfg(test)]
mod tests6 {
    use super::*;

    #[test]
    fn test6() {
        let p1 = Point2 { x: 5, y: 10 };
        let p2 = Point2 { x: "Hello", y: '中' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
    }
}

struct Point3<T> {
    x: T,
    y: T,
}

impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests7 {
    use super::*;

    #[test]
    fn test7() {
        let p = Point3 { x: 5.0, y: 10.0 };
        println!("{}", p.distance_from_origin());
    }
}

Chapter 10.2

struct Array<T, const N: usize> {
    data: [T; N],
}

fn test1() {
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1.0, 2.0, 3.0],
        },
        Array {
            data: [1, 2],
        },
    ];

    println!("Success!");
}

fn print_array<T>(arr: T) 
where
    T: std::fmt::Debug, 
{
    println!("{:?}", arr);
}

fn test2() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::mem;

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    // Placeholder for implementation, this function checks the size
}

// Fix the errors in main.
fn test3() {
    check_size([0u8; 767]); // Size of u8 array (767 bytes)
    check_size([0i32; 191]); // Size of i32 array (191 * 4 bytes = 764 bytes)
    
    check_size(["hello你好"; 10]); // Size of &str array: 10 * size of &str (each &str has 1 byte per character in UTF-8, approx 10 * 12 bytes)
    check_size([(); 100].map(|_| "hello你好".to_string()));  // Size of String array (100 strings of length 6)
    check_size(['中'; 10]); // Size of char array (10 * 4 bytes = 40 bytes)

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

Chapter 10.3

// test1
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        "I'm a good student".to_string()
    }
}

struct Teacher {}
impl Hello for Teacher {
    fn say_something(&self) -> String {
        "I'm not a bad teacher".to_string()
    }
}


fn test1() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}


struct Centimeters(f64);


struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}


struct Seconds(i32);


fn test2() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}


fn multiply<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}


fn test3() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}


use std::ops;

struct Foo;
struct Bar;

struct FooBar;

struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}


fn test4() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}


trait Summary {
    fn summarize(&self) -> String;
}


struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}


struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}


fn test5() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(post);
    summary(weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}


struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

fn random_animal(random_number: f64) -> impl Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        Cow {}
    }
}

#[test]
fn test6() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}


fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[test]
fn test7() {
    assert_eq!(sum(1, 2), 3);
}

// test8
fn example1() {
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 16);
}

#[test]
fn test8() {
    example1();
}

Chapter 10.4

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
} 

trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn test1() {
    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn test2() {
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

fn hatch_a_bird(n: i32) -> Box<dyn Bird> {
    if n == 1 {
        Box::new(Swan)
    } else {
        Box::new(Duck)
    }
}

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn test3() {
    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));
    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn static_dispatch(x: impl Foo) {
    println!("{}", x.method());
}

fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}", x.method());
}

fn test4() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}

trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function(x: Box<dyn MyTrait>)  {
    x.f()
}

fn test5() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}



Chapter 10.5

use std::fmt;
use std::ops::Sub;

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

fn test1() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn test2() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!");
}

trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn test3() {
    let person = Human;

    assert_eq!(person.fly(), "This is your captain speaking.");
    assert_eq!(person.fly(), "Up!");

    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!");
}

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}

impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn test4() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    println!("{}", comp_sci_student_greeting(&student));
}

struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn test5() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}

