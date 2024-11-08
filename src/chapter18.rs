Chapter 18.1
fn test1() {
    let color = String::from("green");

    let print = move || println!("`color`: {}", color);

    print();
    print();

    let _reborrow = &color;

    println!("{}", color);
}

fn test2() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    let _reborrow = &count; 

    inc();

    let _count_reborrowed = &mut count; 

    assert_eq!(count, 0);
}

fn test3() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable);
    };

    consume();
    consume();
}

fn take<T>(_v: T) {}

fn test4() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}

fn test5() {
    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    let x = vec![1, 2, 3];
    fn_once(|z| { z == x.len() })
}

fn test6() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

fn exec<'a, F: FnOnce(&str)>(mut f: F) {
    f("hello")
}

fn test7() {
    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        std::mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

fn test8() {
    let mut s = String::new();

    let update_string = |str| -> String { s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&str) -> String>(mut f: F) {
    f("hello");
}

fn test9() {
    fn call_me<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

fn test10() {
    fn create_fn() -> impl Fn(i32) -> i32 {
        let num = 5;
        move |x| x + num
    }

    let fn_plain = create_fn();
    fn_plain(1);
}

fn test11() {
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x + num)
        }
    }
}

Chapter 18.2

#[test]
fn test1() {
    let arr = [0; 10];
    for i in arr.iter() {
        println!("{}", i);
    }
}

#[test]
fn test2() {
    let mut v = Vec::new();
    for n in 0..100 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}

#[test]
fn test3() {
    let mut v1 = vec![1, 2];

    assert_eq!(v1.pop(), Some(2));
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.pop(), None);
}

#[test]
fn test4() {
    let arr = vec![0; 10];
    for i in arr.into_iter() {
        println!("{}", i);
    }
}

#[test]
fn test5() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

#[test]
fn test6() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}

#[test]
fn test7() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_curr = self.curr + self.next;
            self.curr = self.next;
            self.next = new_curr;
            Some(self.curr)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}

#[test]
fn test8() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.clone().sum();

    assert_eq!(total, 6);

    println!("{:?}, {:?}", v1, v1_iter);
}

#[test]
fn test9() {
    use std::collections::HashMap;
    let names = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<_, _> = names.iter().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().collect::<Vec<_>>();

    assert_eq!(v2, vec![&1, &2, &3]);
}

#[test]
fn test10() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn test11() {
    use std::collections::HashMap;
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}", folks);
}

#[test]
fn test12() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
