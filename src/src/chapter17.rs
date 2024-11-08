Chapter 17.1
fn test1() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}

fn test2(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test3<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn test4<'a>() {
    let _x = 12;
    let y: &'a i32 = &_x;
}

fn test5() {
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn test6() {
    let var_a = 35;
    let example: Example;
    {
        let var_b = NoCopyType {};
        example = Example { a: &var_a, b: &var_b };
    }
    println!("(Success!) {:?}", example);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn fix_me(foo: &Example) -> &NoCopyType {
    foo.b
}

fn test7() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!");
}

struct ImportantExcerpt {
    part: &str,
}

impl ImportantExcerpt {
    fn level<'a>(&'a self) -> i32 {
        3
    }
}

fn test8() {}

fn input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn test9() {}

fn test10() {}

Chapter 17.2

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut config: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    Some(&mut Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })
}

fn test1() {
    unsafe {
        config = init();
        println!("{:?}", config)
    }
}

fn test2() {
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

fn test3() {
    use std::fmt::Debug;

    fn print_it<T: Debug + 'static>(input: T) {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it1(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it2<T: Debug + 'static>(input: &T) {
        println!("'static value passed in is: {:?}", input);
    }

    let i = 5;
    print_it(i);
    print_it(&i);
    print_it1(&i);
    print_it2(&i);
}

fn test4() {
    use std::fmt::Display;

    fn print_a<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }

    fn print_b<T>(t: &T)
    where
        T: Display + 'static,
    {
        println!("{}", t);
    }

    fn print_c(t: &'static dyn Display) {
        println!("{}", t)
    }

    fn print_d(t: &'static impl Display) {
        println!("{}", t)
    }

    fn print_e(t: &(dyn Display + 'static)) {
        println!("{}", t)
    }

    fn print_f(t: &(impl Display + 'static)) {
        println!("{}", t)
    }

    fn print_g(t: &'static String) {
        println!("{}", t);
    }

    let mut string = "First".to_owned();
    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string);
    print_d(&string);
    print_e(&string);
    print_f(&string);
    print_g(&string);
}

fn test5() {
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);
}

fn test6() {
    fn need_static(r: &'static str) {
        assert_eq!(r, "hello");
    }
    let v = "hello";
    need_static(v);
    println!("Success!");
}

Chapter 17.3

struct DoubleRef<'r, 's, T> {
    r: &'r T,
    s: &'s T,
}

fn test1() {
    println!("Success!");
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn test2() {
    println!("Success!");
}

fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) {
    y = x;
    let r: &'b &'a i32 = &&0;
}

fn test3() {
    println!("Success!");
}

fn call_on_ref_zero<'a, F>(f: F)
where
    F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn test4() {
    println!("Success!");
}

fn test5() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref1 += 1;
    *ref2 += 2;

    println!("{}", data);
}

fn test6() {
    struct Interface<'a> {
        manager: &'a mut Manager<'a>,
    }

    impl<'a> Interface<'a> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str,
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a> List<'a> {
        pub fn get_interface(&'a mut self) -> Interface {
            Interface {
                manager: &mut self.manager,
            }
        }
    }

    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
